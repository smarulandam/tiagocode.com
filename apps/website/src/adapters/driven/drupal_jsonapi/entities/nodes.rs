use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_bool_from_anything;

use crate::adapters::driven::drupal_jsonapi::entities::{
    ContentField, ImageField, MetatagsField, PathField, TagsVocabulary,
};

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ArticleNode {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    title: String,
    body: String,
    content: Vec<ContentField>,
    path: PathField,
    thumbnail: ImageField,
    tags: TagsVocabulary,
    metatags: Vec<MetatagsField>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
    #[serde(rename = "created")]
    created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PortfolioNode {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    title: String,
    path: PathField,
    content: Vec<ContentField>,
    metatags: Vec<MetatagsField>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
    #[serde(rename = "created")]
    created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PageNode {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    title: String,
    body: String,
    path: PathField,
    metatags: Vec<MetatagsField>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
    #[serde(rename = "created")]
    created_at: String,
}
