use async_trait::async_trait;
use std::any::type_name;
use std::time::Duration;

use crate::adapters::driven::drupal_jsonapi::entities::SitemapEntriesCollection;
use crate::adapters::driven::drupal_jsonapi::entities::SitemapNode;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalSitemapAdapter;
use crate::adapters::driven::drupal_jsonapi::mappers::SitemapEntryMapper;
use crate::adapters::driven::drupal_jsonapi::services::JsonApiClientService;
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::sitemap::Sitemap;
use crate::application::ports::driven::ForFetchingSitemapEntries;
use crate::helpers::{Cache, Http};

pub struct SitemapRepository {
    cache_client: Box<Cache>,
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<dyn ExternalSitemapAdapter<Input = SitemapNode>>,
}

impl SitemapRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            cache_client: Box::new(cache_client.clone()),
            api_client: Box::new(JsonApiClientService::new(http_client)),
            api_adapter: Box::new(SitemapEntryMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingSitemapEntries for SitemapRepository {
    async fn find_all(&self) -> Result<Sitemap> {
        let endpoint = "/api/sitemap/entries";

        self.cache_client
            .remember(endpoint, Duration::from_days(7), || async {
                let external_entries = self
                    .api_client
                    .get_external_data::<SitemapEntriesCollection>(endpoint)
                    .await
                    .map_err(|e| AppError::external(type_name::<Self>(), e))?;

                self.api_adapter
                    .adapt_multiple(external_entries.data().clone())
            })
            .await
    }
}
