use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::adapters::driven::drupal_jsonapi::entities::{ArticleNode, PageNode};
use crate::adapters::driven::drupal_jsonapi::entities::{PortfolioNode, TagsVocabulary};

pub type NodePageResource = JsonApiDocument<PageNode>;
pub type NodePortfolioResource = JsonApiDocument<PortfolioNode>;
pub type NodeArticleResource = JsonApiDocument<ArticleNode>;
pub type NodeArticleCollection = JsonApiDocument<Vec<ArticleNode>>;
pub type VocabularyTagCollection = JsonApiDocument<Vec<TagsVocabulary>>;

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct JsonApiDocument<T> {
    jsonapi: JsonApiMetaData,
    data: T,
    links: JsonApiLinks,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
struct JsonApiMetaData {
    version: String,
    parsed: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
struct JsonApiLinks {
    #[serde(rename = "self")]
    current: JsonApiLink,
    #[serde(rename = "next")]
    next: Option<JsonApiLink>,
    #[serde(rename = "prev")]
    prev: Option<JsonApiLink>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
struct JsonApiLink {
    href: String,
}
