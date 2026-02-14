use async_trait::async_trait;
use std::any::type_name;

use crate::adapters::driven::drupal_jsonapi::entities::{NodePortfolioResource, PortfolioNode};
use crate::adapters::driven::drupal_jsonapi::mappers::ExternalPortfolioAdapter;
use crate::adapters::driven::drupal_jsonapi::mappers::PortfolioNodeMapper;
use crate::adapters::driven::drupal_jsonapi::services::JsonApiClientService;
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::portfolio::Portfolio;
use crate::application::ports::driven::ForFetchingPortfolioData;
use crate::helpers::{Cache, Http};

const RESOURCE_QUERY: &str = "\
    include=content,content.document.media_document,content.image.media_image,content.items.media.media_image,content.items.items\
    &jsonapi_include=1";

/// Repository for fetching and transforming portfolio data from an external API.
///
/// This struct implements the `ForFetchingPortfolioData` output port of the hexagonal architecture
/// by integrating with a CMS API client to retrieve portfolio items and transform them into domain entities.
pub struct PortfolioRepository {
    api_client: Box<JsonApiClientService>,
    api_adapter: Box<dyn ExternalPortfolioAdapter<Input = PortfolioNode>>,
}

impl PortfolioRepository {
    pub fn new(http_client: Http, cache_client: Cache) -> Self {
        Self {
            api_client: Box::new(JsonApiClientService::new(http_client, cache_client)),
            api_adapter: Box::new(PortfolioNodeMapper::default()),
        }
    }
}

#[async_trait(?Send)]
impl ForFetchingPortfolioData for PortfolioRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Portfolio> {
        let adapter = type_name::<Self>();
        let endpoint = self
            .api_client
            .resolve_external_endpoint(slug)
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        let endpoint = format!("{endpoint}?{RESOURCE_QUERY}");

        let portfolio = self
            .api_client
            .get_external_data::<NodePortfolioResource>(&endpoint)
            .await
            .map_err(|e| AppError::external(adapter, e))?;

        Ok(self.api_adapter.adapt(portfolio.data().clone())?)
    }
}

#[cfg(test)]
pub mod tests {
    // use super::*;
    // use mockito::Matcher::Regex;
    // use mockito::{Mock, Server};
    //
    // use crate::adapters::driven::drupal_jsonapi::services::http_client_service::tests::http_client_mock;
    // use crate::application::value_objects::ModerationStatus;

    // #[actix_rt::test]
    // async fn succeeds_to_fetch_portfolio_when_slug_is_valid() {
    //     let mut server = Server::new_async().await;
    //
    //     let router_mock = router_mock(&mut server, "/portfolio/john-doe", 200).await;
    //     let resource_mock = resource_mock(&mut server, "/jsonapi/node/portfolio", 200).await;
    //     let http_client_mock = http_client_mock(&server.url());
    //
    //     let portfolio = PortfolioRepository::new(http_client_mock)
    //         .find_by_slug("/portfolio/john-doe")
    //         .await
    //         .unwrap();
    //
    //     assert!(router_mock.matched_async().await);
    //     assert!(resource_mock.matched_async().await);
    //     assert_eq!(portfolio.title().as_str(), "John Doe");
    //     assert_eq!(*portfolio.status(), ModerationStatus::Published);
    //     assert!(portfolio.sections().len() > 1);
    // }
    //
    // #[actix_rt::test]
    // async fn fails_to_fetch_portfolio_when_slug_is_invalid() {
    //     let mut server = Server::new_async().await;
    //
    //     let router_mock = router_mock(&mut server, "/non-existing-portfolio", 404).await;
    //     let resource_mock = resource_mock(&mut server, "/jsonapi/node/portfolio", 200).await;
    //     let http_client_mock = http_client_mock(&server.url());
    //
    //     let result = PortfolioRepository::new(http_client_mock)
    //         .find_by_slug("/non-existing-portfolio")
    //         .await;
    //
    //     assert!(router_mock.matched_async().await);
    //     assert!(!resource_mock.matched_async().await);
    //     assert!(result.is_err());
    // }
    //
    // async fn router_mock(server: &mut Server, path: &str, status: usize) -> Mock {
    //     let path = format!("/router/translate-path?path={path}");
    //     let body_file = &format!("tests/fixtures/http_decoupled_router_{status}.json");
    //
    //     server
    //         .mock("GET", path.as_str())
    //         .with_status(status)
    //         .with_header("content-type", "application/json")
    //         .with_body_from_file(body_file)
    //         .create_async()
    //         .await
    // }
    //
    // async fn resource_mock(server: &mut Server, path: &str, status: usize) -> Mock {
    //     let body_file_path = &format!("tests/fixtures/http_portfolio_resource_{status}.json");
    //
    //     server
    //         .mock("GET", Regex(path.to_string()))
    //         .with_status(status)
    //         .with_header("content-type", "application/vnd.api+json")
    //         .with_body_from_file(body_file_path)
    //         .create_async()
    //         .await
    // }
}
