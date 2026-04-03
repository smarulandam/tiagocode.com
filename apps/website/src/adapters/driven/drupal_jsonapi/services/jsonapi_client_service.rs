use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_value;

use crate::adapters::driven::drupal_jsonapi::entities::ResolvedRoute;
use crate::application::domain::core::{AppError, Result};
use crate::helpers::Http;

pub struct JsonApiClientService {
    http_service: Http,
}

impl JsonApiClientService {
    pub fn new(http_service: Http) -> Self {
        Self { http_service }
    }

    pub async fn resolve_external_endpoint(&self, language: &str, path: &str) -> Result<String> {
        let endpoint = translated_route_endpoint(language, path);
        let route_data = self.http_service.get_json(&endpoint).await?;
        let route_data = from_value::<ResolvedRoute>(route_data)
            .map_err(|e| AppError::decode("resolved route", e))?;

        Ok(format!(
            "/{}/{}/{}/{}",
            route_data.jsonapi().path_prefix(),
            route_data.entity().entity_type(),
            route_data.entity().bundle(),
            route_data.entity().uuid()
        ))
    }

    pub async fn get_external_data<T>(&self, endpoint: &str) -> Result<T>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug,
    {
        let data = self.http_service.get_json(endpoint).await?;

        let data = serde_json_path_to_error::from_value::<T>(data)
            .map_err(|e| AppError::decode("jsonapi payload", e))?;

        Ok(data)
    }
}

fn translated_route_endpoint(language: &str, path: &str) -> String {
    let is_spanish = language == "es";

    let path = if matches!(path, "/en" | "/es") {
        path
    } else if is_spanish {
        path.strip_prefix("/es").unwrap_or(path)
    } else {
        path.strip_prefix("/en").unwrap_or(path)
    };

    let path = if path.is_empty() { "/" } else { path };

    let path = if is_spanish {
        format!("/es/router/translate-path?path={path}")
    } else {
        format!("/router/translate-path?path={path}")
    };

    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translated_route_endpoint_removes_english_prefix_from_path() {
        let endpoint = translated_route_endpoint("en", "/en/articles/my-post");

        assert_eq!(endpoint, "/router/translate-path?path=/articles/my-post");
    }

    #[test]
    fn translated_route_endpoint_uses_spanish_router_prefix_and_strips_path_prefix() {
        let endpoint = translated_route_endpoint("es", "/es/articulos/mi-post");

        assert_eq!(
            endpoint,
            "/es/router/translate-path?path=/articulos/mi-post"
        );
    }

    #[test]
    fn translated_route_endpoint_adds_missing_leading_slash() {
        let endpoint = translated_route_endpoint("es", "es");

        assert_eq!(endpoint, "/es/router/translate-path?path=/es");
    }

    #[test]
    fn translated_route_endpoint_keeps_exact_english_home_path() {
        let endpoint = translated_route_endpoint("en", "/en");

        assert_eq!(endpoint, "/router/translate-path?path=/en");
    }

    #[test]
    fn translated_route_endpoint_keeps_exact_spanish_home_path() {
        let endpoint = translated_route_endpoint("es", "/es");

        assert_eq!(endpoint, "/es/router/translate-path?path=/es");
    }
}
