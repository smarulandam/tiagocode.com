use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct DocumentMedia {
    id: String,
    uri: DocumentUriMetadata,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(rename = "filemime")]
    mime_type: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    filesize: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct DocumentUriMetadata {
    url: String,
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ImageMedia {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    meta: ImageGeneralMetadata,
    image_style_uri: ImageStylesMetadata,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ImageStylesMetadata {
    max_2600x2600: String,
    max_900x550: String,
    medium_500x500: String,
    medium: String,
    thumbnail: String,
    thumbnail_664x410: String,
    thumbnail_260x210: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ImageGeneralMetadata {
    alt: String,
    title: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    width: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    height: u16,
}
