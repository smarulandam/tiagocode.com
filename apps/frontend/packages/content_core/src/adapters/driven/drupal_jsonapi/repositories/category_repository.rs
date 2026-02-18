use async_trait::async_trait;
use std::any::type_name;

use crate::adapters::driven::drupal_jsonapi::entities::{TagsVocabulary, VocabularyTagCollection};
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalCategoryMapper;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalTagsVocabularyMapper;
use crate::adapters::driven::drupal_jsonapi::services::JsonApiClientService;
use crate::application::domain::article::Category;
use crate::application::domain::core::AppError;
use crate::application::ports::driven::ForFetchingCategoriesList;
use crate::helpers::{Cache, Http};

const COLLECTION_QUERY: &str = "\
    &filter[status]=1\
    &page[limit]=10\
    &sort[sort-created][path]=name&sort[sort-created][direction]=asc\
    &jsonapi_include=1";

/// Repository for fetching and transforming category data from an external API.
///
/// This struct implements a data access layer by integrating with a CMS API client
/// to retrieve category information and transform it into domain entities using
/// the adapter pattern.
pub struct CategoryRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<dyn ExternalCategoryMapper<Input = TagsVocabulary>>,
}

impl CategoryRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client, cache_client)),
            api_adapter: Box::new(ExternalTagsVocabularyMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingCategoriesList for CategoryRepository {
    async fn find_all_categories(&self) -> crate::application::domain::core::Result<Vec<Category>> {
        let adapter = type_name::<Self>();
        let endpoint = &format!("/jsonapi/taxonomy_term/tags?{COLLECTION_QUERY}");

        let categories = self
            .api_client
            .get_external_data::<VocabularyTagCollection>(endpoint)
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        Ok(self
            .api_adapter
            .adapt_multiple(categories.data().clone())?
            .into_iter()
            .collect())
    }
}
