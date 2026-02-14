use leptos::prelude::ServerFnError;
use leptos::prelude::*;

use crate::application::domain::portfolio::Portfolio;

#[server]
pub async fn portfolio_detail_controller() -> Result<Portfolio, ServerFnError> {
    use actix_web::web::Data;
    use leptos::logging::error;
    use leptos_actix::extract;

    use crate::adapters::driven::drupal_jsonapi::repositories::ArticleRepository;
    use crate::adapters::driven::drupal_jsonapi::repositories::PortfolioRepository;
    use crate::application::domain::core::AppError;
    use crate::application::ports::driver::ForDisplayingPortfolio;
    use crate::application::use_cases::ShowPortfolioDetailUseCase;
    #[cfg(feature = "ssr")]
    use crate::helpers::Cache;
    #[cfg(feature = "ssr")]
    use crate::helpers::Http;

    let http_client: Data<Http> = extract().await?;
    let http_client = http_client.get_ref();

    let cache_client: Data<Cache> = extract().await?;
    let cache_client = cache_client.get_ref();

    let article_repository = ArticleRepository::new(http_client.clone(), cache_client.clone());
    let portfolio_repository = PortfolioRepository::new(http_client.clone(), cache_client.clone());

    let portfolio_service = ShowPortfolioDetailUseCase::new(
        Box::new(portfolio_repository),
        Box::new(article_repository),
    );

    let portfolio = portfolio_service.execute().await.map_err(|e| {
        error!("{}", e.to_string());
        ServerFnError::<AppError>::ServerError(e.to_string())
    })?;

    Ok(portfolio)
}
