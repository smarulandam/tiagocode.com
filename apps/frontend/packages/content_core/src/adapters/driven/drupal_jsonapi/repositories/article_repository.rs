use async_trait::async_trait;
use std::any::type_name;

use crate::adapters::driven::drupal_jsonapi::entities::NodeArticleResource;
use crate::adapters::driven::drupal_jsonapi::entities::{ArticleNode, NodeArticleCollection};
use crate::adapters::driven::drupal_jsonapi::mappers::ArticleNodeMapper;
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalArticleMapper;
use crate::adapters::driven::drupal_jsonapi::services::JsonApiClientService;
use crate::application::domain::article::{Article, Articles};
use crate::application::domain::core::{AppError, Result};
use crate::application::ports::driven::ForFetchingArticlesFeatured;
use crate::application::ports::driven::{ForFetchingArticleData, ForFetchingArticlesList};
use crate::helpers::{Cache, Http};

const COLLECTION_QUERY: &str = "\
    include=tags,content.media.media_image,thumbnail.media_image,content.media_list.media_image\
    &filter[status]=1&sort=created,title\
    &page[limit]=2\
    &sort[sort-created][path]=created&sort[sort-created][direction]=desc\
    &jsonapi_include=1";

const RESOURCE_QUERY: &str = "\
    include=tags,content.media.media_image,thumbnail.media_image,content.media_list.media_image\
    &jsonapi_include=1";

/// Repository for fetching and transforming article data from an external CMS API.
///
/// This struct implements the `ForFetchingArticleData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve portfolio items and transform them into domain entities.
///
/// # Dependencies
/// - `api_client`: A connector to the CMS API for fetching portfolio data
/// - `api_mapper`: A mapper to convert external portfolio items into domain `Article` objects
pub struct ArticleRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<dyn ExternalArticleMapper<Input = ArticleNode>>,
}

impl ArticleRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client, cache_client)),
            api_adapter: Box::new(ArticleNodeMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingArticlesFeatured for ArticleRepository {
    async fn get_featured(&self) -> Result<Articles> {
        let endpoint =
            &format!("/jsonapi/node/article?{COLLECTION_QUERY}&filter[promoted]=1&page[limit]=2");

        let external_articles = self
            .api_client
            .get_external_data::<NodeArticleCollection>(endpoint)
            .await
            .map_err(|e| AppError::external(type_name::<Self>(), e))?;

        let articles = self
            .api_adapter
            .adapt_multiple(external_articles.data().clone())?;

        Ok(articles)
    }
}

#[async_trait(?Send)]
impl ForFetchingArticlesList for ArticleRepository {
    async fn get_list(&self, category_id: Option<String>) -> Result<Vec<Article>> {
        let adapter = type_name::<Self>();
        let mut endpoint = format!("/jsonapi/node/article?{COLLECTION_QUERY}&page[limit]=10");

        if let Some(category) = category_id {
            endpoint.push_str(&format!("&filter[tags][condition][path]=tags.machine_name&filter[tags][condition][value]={category}"));
        }

        let articles = self
            .api_client
            .get_external_data::<NodeArticleCollection>(endpoint.as_str())
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        Ok(self
            .api_adapter
            .adapt_multiple(articles.data().clone())?
            .into_iter()
            .collect())
    }
}

#[async_trait(?Send)]
impl ForFetchingArticleData for ArticleRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Article> {
        let adapter = type_name::<Self>();
        let endpoint = self
            .api_client
            .resolve_external_endpoint(slug)
            .await
            .map_err(|e| AppError::external(adapter, e))?;
        let endpoint = &format!("{endpoint}?{RESOURCE_QUERY}");

        let article = self
            .api_client
            .get_external_data::<NodeArticleResource>(endpoint)
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        Ok(self.api_adapter.adapt(article.data().clone())?)
    }
}

#[cfg(test)]
pub mod tests {}
