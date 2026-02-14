use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::adapters::driven::drupal_jsonapi::entities::{
    ContentField, DocumentField, ImageField, LinkField,
};

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PortfolioResumeParagraph {
    id: String,
    subtitle: String,
    text: String,
    title: String,
    items: Vec<ContentField>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PortfolioAboutMeParagraph {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    text_list: Vec<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    years_of_experience: u8,
    image: ImageField,
    document: DocumentField,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PortfolioProjectsParagraph {
    id: String,
    title: String,
    subtitle: String,
    text: String,
    items: Vec<ContentField>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PortfolioArticlesParagraph {
    id: String,
    title: String,
    subtitle: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ContentTimelineParagraph {
    id: String,
    items: Vec<ContentTimelineItemParagraph>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ContentTimelineItemParagraph {
    id: String,
    title: String,
    subtitle: String,
    text: Option<String>,
    #[serde(rename = "date_human_readable")]
    date4human: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ContentHoverCardParagraph {
    id: String,
    title: String,
    text: Option<String>,
    link: LinkField,
    media: ImageField,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ContentTextParagraph {
    id: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ContentMediaParagraph {
    id: String,
    media: ImageField,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ContentSliderParagraph {
    id: String,
    media_list: Vec<ImageField>,
}
