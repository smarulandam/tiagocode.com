use reqwest::{Client, Method, RequestBuilder, Response, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::Value;

use crate::application::domain::core::{AppError, Result};

/// Small HTTP client facade with optional base URL and basic auth.
///
/// This wraps `reqwest::Client` with minimal configuration and error mapping.
#[derive(Clone, Debug)]
pub struct Http {
    client: Client,
    base_url: Option<Url>,
    auth: Option<BasicAuth>,
}

#[derive(Clone, Debug)]
pub struct BasicAuth {
    username: SecretString,
    password: SecretString,
}

impl Http {
    pub fn new() -> Self {
        Self {
            base_url: None,
            auth: None,
            client: Client::new(),
        }
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(Url::parse(url).unwrap());
        self
    }

    pub fn basic_auth(mut self, username: &str, password: &str) -> Self {
        self.auth = Some(BasicAuth::from((username, password)));
        self
    }

    pub async fn get_json(&self, url: &str) -> Result<Value> {
        self.request(Method::GET, url)
            .await
            .map_err(|e| AppError::external("http", e))?
            .json::<Value>()
            .await
            .map_err(|e| AppError::decode("http", e))
    }

    fn resolve_url(&self, path: &str) -> Result<Url> {
        if let Ok(url) = Url::parse(path) {
            if !url.scheme().is_empty() {
                return Ok(url);
            }
        }

        self.base_url
            .as_ref()
            .ok_or_else(|| AppError::validation("http", format!("cannot resolve `{}`", path)))?
            .join(path)
            .map_err(|e| AppError::validation("http", e.to_string()))
    }

    async fn request(&self, method: Method, endpoint: &str) -> Result<Response> {
        let mut request = self.client.request(method, self.resolve_url(endpoint)?);

        if let Some(auth) = &self.auth {
            request = auth.apply(request);
        }

        let response = request
            .send()
            .await
            .map_err(|e| AppError::external("http", e))?;

        if response.status().is_success() {
            return Ok(response);
        }

        let response = response
            .text()
            .await
            .unwrap_or_else(|_| "failed to read body".to_string());

        Err(AppError::external("http", response))
    }
}

impl BasicAuth {
    /// Applies the credentials to a reqwest request builder.
    ///
    /// This keeps auth handling centralized in one place.
    fn apply(&self, request: RequestBuilder) -> RequestBuilder {
        request.basic_auth(
            self.username.expose_secret().to_string(),
            Some(self.password.expose_secret().to_string()),
        )
    }
}

impl From<(&str, &str)> for BasicAuth {
    /// Creates BasicAuth from a `(username, password)` pair.
    ///
    /// This is a convenience for builder-style configuration.
    fn from(value: (&str, &str)) -> Self {
        let (username, password) = value;
        Self {
            username: SecretString::new(Box::from(username.to_string())),
            password: SecretString::new(Box::from(password.to_string())),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use mockito::{Mock, Server};

    #[test]
    fn creation_succeeds_when_using_default_values() {
        let http_client = Http::new();
        assert!(http_client.base_url.is_none());
        assert!(http_client.auth.is_none());
    }

    #[test]
    fn creation_succeeds_when_custom_config_is_valid() {
        let http_client_mock = http_client_mock("https://localhost.dev/");
        let base_url = http_client_mock.base_url.unwrap();
        let basic_auth = http_client_mock.auth.unwrap();

        assert_eq!(base_url.as_str(), "https://localhost.dev/");
        assert_eq!(basic_auth.username.expose_secret(), "test");
        assert_eq!(basic_auth.password.expose_secret(), "123456");
    }

    #[test]
    fn endpoint_resolver_succeeds_when_input_is_absolute_url() {
        let request_endpoint = http_client_mock("https://baseurl.dev")
            .resolve_url("https://localhost.dev/api")
            .unwrap();

        // base url is ignored, as it's an absolute URL
        assert_eq!(request_endpoint.as_str(), "https://localhost.dev/api");
    }

    #[test]
    fn endpoint_resolver_succeeds_when_input_is_relative_url() {
        let request_endpoint = http_client_mock("https://localhost.dev")
            .resolve_url("/api")
            .unwrap();

        assert_eq!(request_endpoint.as_str(), "https://localhost.dev/api");
    }

    #[test]
    fn endpoint_resolver_fails_when_input_is_invalid() {
        let result = Http::new().resolve_url("..invalid..url..");

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn request_succeeds_when_endpoint_is_valid() {
        let mut server = Server::new_async().await;
        let http_client_mock = http_client_mock(&server.url());
        let request_mock = request_mock(&mut server, "/api/v1/resource", 200).await;
        let response = http_client_mock.get_json("/api/v1/resource").await;

        assert!(response.is_ok());
        assert!(request_mock.matched_async().await);
    }

    #[actix_rt::test]
    async fn request_fails_when_external_service_returns_error() {
        let mut server = Server::new_async().await;
        let http_client_mock = http_client_mock(&server.url());
        let request_mock = request_mock(&mut server, "/api/v1/resource", 500).await;
        let response = http_client_mock.get_json("/api/v1/resource").await;

        assert!(response.is_err());
        assert!(request_mock.matched_async().await);
    }

    #[actix_rt::test]
    async fn request_ignores_basic_auth_header_when_credentials_are_not_configured() {
        let mut server = Server::new_async().await;
        let request_mock = server
            .mock("GET", "/example")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(String::from(r#"{"message": "Hello, World!"}"#))
            .create_async()
            .await;

        let http_client_mock = Http::new().base_url(server.url().as_str());
        let response = http_client_mock.get_json("/example").await;

        assert!(response.is_ok()); // sends request without basic auth
        assert!(request_mock.matched_async().await); // ensures it works
    }

    pub fn http_client_mock(url: &str) -> Http {
        Http::new().base_url(url).basic_auth("test", "123456")
    }

    async fn request_mock(server: &mut Server, path: &str, status: usize) -> Mock {
        server
            .mock("GET", path)
            .with_status(status)
            .with_header("Content-Type", "application/json")
            .with_body_from_file(format!("tests/fixtures/basic_request_{status}.json").to_string())
            .match_header("Authorization", "Basic dGVzdDoxMjM0NTY=")
            .create_async()
            .await
    }
}
