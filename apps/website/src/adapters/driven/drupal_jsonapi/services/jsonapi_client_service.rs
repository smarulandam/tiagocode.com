use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_value;
use std::time::Duration;

use crate::adapters::driven::drupal_jsonapi::entities::ResolvedRoute;
use crate::application::domain::core::{AppError, Result};
use crate::helpers::{Cache, Http};

pub struct JsonApiClientService {
    http_service: Http,
    cache_service: Cache,
}

impl JsonApiClientService {
    pub fn new(http_service: Http, cache_service: Cache) -> Self {
        Self {
            http_service,
            cache_service,
        }
    }

    pub async fn resolve_external_endpoint(&self, path: &str) -> Result<String> {
        let path = format!("/router/translate-path?path={path}");

        let route_data: ResolvedRoute = self
            .cache_service
            .remember(&path, Duration::from_hours(168), || async {
                let route_data = self.http_service.get_json(&path).await?;

                from_value::<ResolvedRoute>(route_data)
                    .map_err(|e| AppError::decode("resolved route", e))
            })
            .await?;

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
        let data: T = self
            .cache_service
            .remember(&endpoint, Duration::from_hours(168), || async {
                let data = self.http_service.get_json(endpoint).await?;

                serde_json_path_to_error::from_value::<T>(data)
                    .map_err(|e| AppError::decode("jsonapi payload", e))
            })
            .await?;

        Ok(data)
    }
}
