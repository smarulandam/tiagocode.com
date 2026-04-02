use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct SitemapNode {
    id: String,
    #[serde(rename = "type")]
    resource_type: String,
    attributes: SitemapAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct SitemapAttributes {
    path: String,
    #[serde(default)]
    lastmod: Option<String>,
}
