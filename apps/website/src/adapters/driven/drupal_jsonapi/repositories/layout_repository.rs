use async_trait::async_trait;
use std::any::type_name;

use crate::adapters::driven::drupal_jsonapi::entities::Navigation;
use crate::adapters::driven::drupal_jsonapi::mappers::{ExternalMenuTreeMapper, NavigationAdapter};
use crate::adapters::driven::drupal_jsonapi::services::JsonApiClientService;
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::layout::MenuTree;
use crate::application::ports::driven::ForFetchingMenuData;
use crate::helpers::{Cache, Http};

/// Repository for fetching and transforming menu data from an external CMS API.
///
/// This struct implements the `ForFetchingMenuData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve menu items and transform them into domain entities.
pub struct LayoutRepository {
    api_client: Box<JsonApiClientService>,
    api_mapper: Box<dyn ExternalMenuTreeMapper<Input = Navigation>>,
}

impl LayoutRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client, cache_client)),
            api_mapper: Box::new(NavigationAdapter::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingMenuData for LayoutRepository {
    async fn find_by_id(&self, id: &str) -> Result<MenuTree> {
        let adapter = type_name::<Self>();
        let menu_tree = self
            .api_client
            .get_external_data::<Navigation>(&format!("/api/menu_items/{id}"))
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        Ok(self.api_mapper.adapt(menu_tree)?)
    }
}
