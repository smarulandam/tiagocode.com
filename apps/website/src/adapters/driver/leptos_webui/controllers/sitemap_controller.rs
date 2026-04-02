use actix_web::{web::Data, HttpResponse};
use leptos::logging::error;

use crate::adapters::driven::drupal_jsonapi::repositories::SitemapRepository;
use crate::application::use_cases::ShowSitemapUseCase;
use crate::helpers::{Cache, Http};

#[actix_web::get("/sitemap.xml")]
pub async fn sitemap(
    http_client: Data<Http>,
    cache_client: Data<Cache>,
    sitemap_public_host: Data<String>,
) -> actix_web::Result<HttpResponse> {
    use crate::application::ports::driver::ForDisplayingSitemap;

    let repository = SitemapRepository::new(
        http_client.get_ref().clone(),
        cache_client.get_ref().clone(),
    );
    let sitemap_service = ShowSitemapUseCase::new(Box::new(repository));

    let sitemap = sitemap_service.execute().await.map_err(|e| {
        error!("{}", e.to_string());
        actix_web::error::ErrorInternalServerError(e.to_string())
    })?;

    let mut body = String::new();

    for entry in sitemap.entries() {
        let path = entry.location().as_str();

        if path.is_empty() || !path.starts_with('/') || path.starts_with("//") {
            continue;
        }

        let location = format!("{}{path}", sitemap_public_host.get_ref());
        let last_modification = entry
            .last_modification()
            .as_ref()
            .map(|l| format!("<lastmod>{}</lastmod>", l.to_rfc3339()))
            .unwrap_or_default();

        body.push_str(&format!(
            "<url><loc>{location}</loc>{last_modification}</url>"
        ));
    }

    Ok(HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
                {body}
            </urlset>
            "#
        )))
}
