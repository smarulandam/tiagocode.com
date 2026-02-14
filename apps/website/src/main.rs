#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::middleware::Compress;
    use actix_web::*;
    use dotenvy::dotenv;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use redis::Client as RedisClient;
    use std::env;

    use website::adapters::driver::leptos_webui::views::app::*;
    use website::helpers::{Cache, Http};

    dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        let api_base_url = env::var("JSONAPI_BASE_URL").expect("JSONAPI_BASE_URL is undefined");
        let api_username = env::var("JSONAPI_USERNAME").expect("JSONAPI_USERNAME is undefined");
        let api_password =
            env::var("JSONAPI_PASSWORD").expect("JSONAPI_PASSWORD is undefined");
        let redis_host = env::var("WEBSITE_REDIS_HOST").expect("WEBSITE_REDIS_HOST is undefined");
        let redis_port = env::var("WEBSITE_REDIS_PORT").expect("WEBSITE_REDIS_PORT is undefined");
        let redis_password =
            env::var("WEBSITE_REDIS_PASSWORD").expect("WEBSITE_REDIS_PASSWORD is undefined");

        let redis_url = format!(
            "rediss://default:{}@{}:{}",
            redis_password, redis_host, redis_port
        );
        let redis_client = RedisClient::open(redis_url.as_str()).unwrap();

        let cache = Cache::new(redis_client);

        let http = Http::new()
            .base_url(api_base_url.as_str())
            .basic_auth(api_username.as_str(), api_password.as_str());

        println!("listening on http://{}", &addr);

        App::new()
            .wrap(Compress::default())
            .service(favicon)
            .service(health)
            .service(cache_purge)
            .service(Files::new("/assets", &site_root))
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <link rel="preconnect" href="https://fonts.googleapis.com" />
                                <link rel="preconnect" href=api_base_url.clone() />
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(cache.to_owned()))
            .app_data(web::Data::new(http.to_owned()))
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
        .bind(&addr)?
        .run()
        .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("/health")]
async fn health() -> actix_web::HttpResponse {
    let api_url = std::env::var("JSONAPI_BASE_URL").unwrap();

    if reqwest::get(api_url).await.unwrap().status().is_success() {
        return actix_web::HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
            );
    }

    actix_web::HttpResponse::InternalServerError().finish()
}

#[cfg(feature = "ssr")]
#[actix_web::post("/internal/cache/purge")]
async fn cache_purge(
    request: actix_web::HttpRequest,
    cache: actix_web::web::Data<website::helpers::Cache>,
) -> actix_web::HttpResponse {
    let expected_token = match std::env::var("WEBSITE_CACHE_PURGE_TOKEN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return actix_web::HttpResponse::InternalServerError()
                .content_type(actix_web::http::header::ContentType::plaintext())
                .body("WEBSITE_CACHE_PURGE_TOKEN is not configured");
        }
    };

    let provided_token = request
        .headers()
        .get("x-webhook-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if provided_token != expected_token {
        return actix_web::HttpResponse::Unauthorized()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body("Invalid webhook token");
    }

    match cache.clear_all().await {
        Ok(()) => actix_web::HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body("Cache purged"),
        Err(error) => actix_web::HttpResponse::InternalServerError()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body(error.to_string()),
    }
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use website::adapters::driver::leptos_webui::views::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
