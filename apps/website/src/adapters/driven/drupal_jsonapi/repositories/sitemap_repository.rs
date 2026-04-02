use async_trait::async_trait;
use std::any::type_name;

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
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<dyn ExternalSitemapAdapter<Input = SitemapNode>>,
}

impl SitemapRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client, cache_client)),
            api_adapter: Box::new(SitemapEntryMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingSitemapEntries for SitemapRepository {
    async fn find_all(&self) -> Result<Sitemap> {
        let external_entries = self
            .api_client
            .get_external_data::<SitemapEntriesCollection>("/api/sitemap/entries")
            .await
            .map_err(|e| AppError::external(type_name::<Self>(), e))?;

        self.api_adapter
            .adapt_multiple(external_entries.data().clone())
    }
}
