use crate::adapters::driven::drupal_jsonapi::entities::deserialize_number_from_string;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

pub type Navigation = Vec<NavigationItem>;

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct NavigationItem {
    key: Option<String>,
    title: String,
    uri: String,
    absolute: String,
    relative: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    weight: isize,
    uuid: Option<String>,
    enabled: bool,
    expanded: bool,
    external: bool,
    field_image: Option<NavigationImageField>,
}

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct NavigationImageField {
    field_media_image: Vec<NavigationImageMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct NavigationImageMetadata {
    target_id: String,
    target_uuid: String,
    target_type: String,
    url: String,
    alt: String,
    title: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    width: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    height: u16,
}
