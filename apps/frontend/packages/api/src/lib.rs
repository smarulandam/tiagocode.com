use dioxus::prelude::*;

use content_core::application::domain::article::{Article, Category};
#[cfg(feature = "server")]
use content_core::application::domain::core::AppError;
use content_core::application::domain::layout::Layout;
use content_core::application::domain::page::Page;
use content_core::application::domain::portfolio::Portfolio;

#[cfg(feature = "server")]
use content_core::adapters::driven::drupal_jsonapi::repositories::{
    ArticleRepository, CategoryRepository, LayoutRepository, PageRepository, PortfolioRepository,
};
#[cfg(feature = "server")]
use content_core::application::ports::driver::{
    ForDisplayingArticle, ForDisplayingArticlesList, ForDisplayingLayout, ForDisplayingPortfolio,
};
#[cfg(feature = "server")]
use content_core::application::use_cases::{
    GetLayoutUseCase, ShowArticleDetailUseCase, ShowArticlesListUseCase, ShowPortfolioDetailUseCase,
};
#[cfg(feature = "server")]
use content_core::helpers::{Cache, Http};
#[cfg(feature = "server")]
use dioxus::prelude::{dioxus_fullstack, StatusCode};
#[cfg(feature = "server")]
use once_cell::sync::OnceCell;
#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
#[derive(Clone, Debug)]
struct ApiConfig {
    jsonapi_base_url: String,
    jsonapi_username: String,
    jsonapi_password: String,
    website_redis_host: String,
    website_redis_port: String,
    website_redis_password: String,
    website_cache_purge_token: String,
}

#[cfg(feature = "server")]
impl ApiConfig {
    fn from_env() -> Result<Self, HttpError> {
        Ok(Self {
            jsonapi_base_url: required_env("JSONAPI_BASE_URL")?,
            jsonapi_username: required_env("JSONAPI_USERNAME")?,
            jsonapi_password: required_env("JSONAPI_PASSWORD")?,
            website_redis_host: required_env("WEBSITE_REDIS_HOST")?,
            website_redis_port: required_env("WEBSITE_REDIS_PORT")?,
            website_redis_password: required_env("WEBSITE_REDIS_PASSWORD")?,
            website_cache_purge_token: required_env("WEBSITE_CACHE_PURGE_TOKEN")?,
        })
    }
}

#[cfg(feature = "server")]
#[derive(Clone, Debug)]
struct ApiState {
    config: ApiConfig,
    http: Http,
    cache: Cache,
}

#[cfg(feature = "server")]
impl ApiState {
    fn from_env() -> Result<Self, HttpError> {
        let config = ApiConfig::from_env()?;

        let redis_url = format!(
            "rediss://default:{}@{}:{}",
            config.website_redis_password, config.website_redis_host, config.website_redis_port
        );

        let redis_client = redis::Client::open(redis_url.as_str()).map_err(|error| {
            HttpError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("invalid redis configuration: {error}"),
            )
        })?;

        let cache = Cache::new(redis_client);

        let http = Http::new()
            .base_url(config.jsonapi_base_url.as_str())
            .basic_auth(
                config.jsonapi_username.as_str(),
                config.jsonapi_password.as_str(),
            );

        Ok(Self {
            config,
            http,
            cache,
        })
    }
}

#[cfg(feature = "server")]
static API_STATE: OnceCell<ApiState> = OnceCell::new();
#[cfg(feature = "server")]
static ENVIRONMENT_READY: OnceCell<()> = OnceCell::new();
#[cfg(feature = "server")]
static CRYPTO_PROVIDER_READY: OnceCell<()> = OnceCell::new();

#[cfg(feature = "server")]
fn state() -> Result<&'static ApiState, HttpError> {
    init_crypto_provider();
    init_environment();
    API_STATE.get_or_try_init(ApiState::from_env)
}

#[cfg(feature = "server")]
fn init_crypto_provider() {
    CRYPTO_PROVIDER_READY.get_or_init(|| {
        // reqwest/redis pull rustls with multiple backends in this workspace.
        // We select one provider explicitly to avoid runtime panics in SSR.
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}

#[cfg(feature = "server")]
fn init_environment() {
    ENVIRONMENT_READY.get_or_init(|| {
        let _ = dotenvy::dotenv();

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let candidates = [
            manifest_dir.join("../../../website/.env"),
            manifest_dir.join("../../../../ops/environment-setup/.env"),
        ];

        for path in candidates {
            if path.exists() {
                let _ = dotenvy::from_path(path);
            }
        }
    });
}

#[cfg(feature = "server")]
fn required_env(name: &'static str) -> Result<String, HttpError> {
    let value = std::env::var(name).map_err(|_| {
        HttpError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{name} is undefined"),
        )
    })?;

    if value.trim().is_empty() {
        return Err(HttpError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("{name} is empty"),
        ));
    }

    Ok(value)
}

#[cfg(feature = "server")]
fn map_app_error(error: AppError) -> HttpError {
    match error {
        AppError::Forbidden { reason } => HttpError::new(StatusCode::FORBIDDEN, reason),
        AppError::NotFound { resource, id } => {
            HttpError::new(StatusCode::NOT_FOUND, format!("{resource} ({id})"))
        }
        AppError::Validation { field, reason } => {
            HttpError::new(StatusCode::BAD_REQUEST, format!("{field}: {reason}"))
        }
        AppError::External { system, message } => HttpError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("external error ({system}): {message}"),
        ),
        AppError::Decode { target, message } => HttpError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("decode error ({target}): {message}"),
        ),
        AppError::Encode { target, message } => HttpError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("encode error ({target}): {message}"),
        ),
        AppError::Unexpected { message } => {
            HttpError::new(StatusCode::INTERNAL_SERVER_ERROR, message)
        }
    }
}

/// Echo endpoint kept for compatibility with existing UI package examples.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, HttpError> {
    Ok(input)
}

#[post("/api/layout")]
pub async fn layout_controller() -> Result<Layout, HttpError> {
    #[cfg(feature = "server")]
    {
        let state = state()?;

        let repository = LayoutRepository::new(state.http.clone(), state.cache.clone());
        let use_case = GetLayoutUseCase::new(Box::new(repository));

        return use_case.execute().await.map_err(map_app_error);
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[post("/api/portfolio")]
pub async fn portfolio_detail_controller() -> Result<Portfolio, HttpError> {
    #[cfg(feature = "server")]
    {
        let state = state()?;

        let article_repository = ArticleRepository::new(state.http.clone(), state.cache.clone());
        let portfolio_repository =
            PortfolioRepository::new(state.http.clone(), state.cache.clone());

        let use_case = ShowPortfolioDetailUseCase::new(
            Box::new(portfolio_repository),
            Box::new(article_repository),
        );

        return use_case.execute().await.map_err(map_app_error);
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[post("/api/articles/list")]
pub async fn articles_list_controller(
    slug: String,
) -> Result<(Page, Vec<Category>, Vec<Article>), HttpError> {
    #[cfg(feature = "server")]
    {
        let state = state()?;

        let page_repository = PageRepository::new(state.http.clone(), state.cache.clone());
        let article_repository = ArticleRepository::new(state.http.clone(), state.cache.clone());
        let category_repository = CategoryRepository::new(state.http.clone(), state.cache.clone());

        let use_case = ShowArticlesListUseCase::new(
            Box::new(article_repository),
            Box::new(category_repository),
            Box::new(page_repository),
        );

        let mut segments = slug.trim_matches('/').split('/');
        let lang = segments.next().unwrap_or("en");
        let section = segments.next().unwrap_or("articles");
        let page_slug = format!("/{lang}/{section}");
        let category = segments.next().map(str::to_owned);

        return use_case
            .execute(page_slug.as_str(), category)
            .await
            .map_err(map_app_error);
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[post("/api/articles/detail")]
pub async fn article_detail_controller(slug: String) -> Result<Article, HttpError> {
    #[cfg(feature = "server")]
    {
        let state = state()?;

        let article_repository = ArticleRepository::new(state.http.clone(), state.cache.clone());

        let use_case = ShowArticleDetailUseCase::new(Box::new(article_repository));

        return use_case.execute(slug.as_str()).await.map_err(map_app_error);
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/health")]
pub async fn health() -> Result<String, HttpError> {
    #[cfg(feature = "server")]
    {
        let state = state()?;

        let response = dioxus_fullstack::reqwest::get(state.config.jsonapi_base_url.as_str())
            .await
            .map_err(|error| {
                HttpError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("health check request failed: {error}"),
                )
            })?;

        if !response.status().is_success() {
            return HttpError::internal_server_error("JSONAPI backend is not healthy");
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| {
                HttpError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to create health timestamp: {error}"),
                )
            })?
            .as_secs()
            .to_string();

        return Ok(timestamp);
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[post("/internal/cache/purge", headers: dioxus_fullstack::HeaderMap)]
pub async fn cache_purge() -> Result<String, HttpError> {
    #[cfg(feature = "server")]
    {
        let state = state()?;

        let provided_token = headers
            .get("x-webhook-token")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();

        if provided_token != state.config.website_cache_purge_token {
            return HttpError::unauthorized("Invalid webhook token");
        }

        state.cache.clear_all().await.map_err(map_app_error)?;

        return Ok("Cache purged".to_string());
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}
