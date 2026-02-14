use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct ResolvedRoute {
    label: String,
    resolved: String,
    entity: RouteEntityMetadata,
    jsonapi: RouteJsonApiMetadata,
    #[serde(rename = "isHomePath")]
    is_home_path: bool,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct RouteEntityMetadata {
    canonical: String,
    #[serde(rename = "type")]
    entity_type: String,
    id: String,
    uuid: String,
    bundle: String,
}

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct RouteJsonApiMetadata {
    individual: String,
    #[serde(alias = "resourceName")]
    resource_name: String,
    #[serde(alias = "pathPrefix")]
    path_prefix: String,
    #[serde(alias = "basePath")]
    base_path: String,
    #[serde(alias = "entryPoint")]
    entry_point: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_resolved_route() {
        let json_str = mock_resolved_route();
        let resolved_route = serde_json::from_str::<ResolvedRoute>(json_str).unwrap();

        assert_eq!(resolved_route.label(), "test");
        assert_eq!(resolved_route.is_home_path().clone(), false);
        assert_eq!(resolved_route.entity().entity_type(), "node");
        assert_eq!(resolved_route.entity().id(), "1");
        assert_eq!(resolved_route.entity().bundle(), "test");
        assert_eq!(resolved_route.jsonapi().resource_name(), "node--test");
        assert_eq!(resolved_route.jsonapi().path_prefix(), "api");
        assert_eq!(resolved_route.jsonapi().base_path(), "/api");
        assert_eq!(
            resolved_route.jsonapi().individual(),
            "https://example.com/api/node/test/550e8400-e29b-41d4-a716-446655440000"
        );
    }

    fn mock_resolved_route() -> &'static str {
        r#"
        {
            "label": "test",
            "isHomePath": false,
            "resolved": "https://example.com/api/node/test",
            "entity": {
                "id": "1",
                "type": "node",
                "bundle": "test",
                "uuid": "550e8400-e29b-41d4-a716-446655440000",
                "canonical": "https://example.com/api/node/test"
            },
            "jsonapi": {
                "individual": "https://example.com/api/node/test/550e8400-e29b-41d4-a716-446655440000",
                "resourceName": "node--test",
                "pathPrefix": "api",
                "basePath": "/api",
                "entryPoint": "https://example.com/api"
            }
        }
        "#
    }
}
