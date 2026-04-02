use actix_web::{web::Bytes, web::Data, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::helpers::Cache;

#[derive(Deserialize)]
struct CachePurgeRequest {
    #[serde(default)]
    paths: Vec<String>,
}

#[actix_web::post("/internal/cache/purge")]
pub async fn cache_purge(request: HttpRequest, body: Bytes, cache: Data<Cache>) -> HttpResponse {
    let expected_token = match std::env::var("WEBSITE_CACHE_PURGE_TOKEN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return HttpResponse::InternalServerError()
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
        return HttpResponse::Unauthorized()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body("Invalid webhook token");
    }

    let paths = if body.is_empty() {
        Vec::new()
    } else {
        let request = match serde_json::from_slice::<CachePurgeRequest>(&body) {
            Ok(request) => request,
            Err(error) => {
                return HttpResponse::BadRequest()
                    .content_type(actix_web::http::header::ContentType::plaintext())
                    .body(format!("Invalid purge payload: {error}"));
            }
        };

        let mut normalized = Vec::new();
        let mut seen = std::collections::BTreeSet::new();

        for path in request.paths {
            let path = path.trim();

            if path.is_empty() || !path.starts_with('/') || path.starts_with("//") {
                continue;
            }

            if seen.insert(path.to_string()) {
                normalized.push(path.to_string());
            }
        }

        normalized
    };

    let result = if paths.is_empty() {
        cache.clear_all().await
    } else {
        cache.clear_paths(&paths).await
    };

    match result {
        Ok(()) => HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body("Cache purged"),
        Err(error) => HttpResponse::InternalServerError()
            .content_type(actix_web::http::header::ContentType::plaintext())
            .body(error.to_string()),
    }
}
