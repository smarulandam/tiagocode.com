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

    pub async fn resolve_external_endpoint(&self, path: &str) -> Result<String> {
        let path = format!("/router/translate-path?path={path}");
        let route_data = self.http_service.get_json(&path).await?;
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
