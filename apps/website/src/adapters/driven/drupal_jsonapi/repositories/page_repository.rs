use async_trait::async_trait;
use std::any::type_name;

use crate::adapters::driven::drupal_jsonapi::entities::{NodePageResource, PageNode};
use crate::adapters::driven::drupal_jsonapi::mappers::{ExternalPageAdapter, PageNodeMapper};
use crate::adapters::driven::drupal_jsonapi::services::JsonApiClientService;
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::page::Page;
use crate::application::ports::driven::ForFetchingPageData;
use crate::helpers::{Cache, Http};

const RESOURCE_QUERY: &str = "jsonapi_include=1";

/// Repository for fetching and transforming portfolio data from an external API.
///
/// This struct implements the `ForFetchingPortfolioData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve portfolio items and transform them into domain entities.
pub struct PageRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<dyn ExternalPageAdapter<Input = PageNode>>,
}

impl PageRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client, cache_client)),
            api_adapter: Box::new(PageNodeMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingPageData for PageRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Page> {
        let adapter = type_name::<Self>();
        let endpoint = self
            .api_client
            .resolve_external_endpoint(slug)
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        let endpoint = format!("{endpoint}?{RESOURCE_QUERY}");

        let portfolio = self
            .api_client
            .get_external_data::<NodePageResource>(&endpoint)
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        Ok(self.api_adapter.adapt(portfolio.data().clone())?)
    }
}
