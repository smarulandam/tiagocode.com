use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::adapters::driven::drupal_jsonapi::entities::PathField;
use serde_aux::field_attributes::deserialize_bool_from_anything;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct TagsVocabulary {
    id: String,
    name: String,
    #[serde(rename = "type")]
    entity_type: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    status: bool,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    weight: usize,
    emoji: String,
    path: PathField,
}
