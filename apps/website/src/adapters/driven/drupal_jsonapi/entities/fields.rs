use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::adapters::driven::drupal_jsonapi::entities::{
    ContentHoverCardParagraph, ContentMediaParagraph, ContentSliderParagraph, ContentTextParagraph,
    ContentTimelineParagraph, DocumentMedia, ImageMedia, PortfolioAboutMeParagraph,
    PortfolioArticlesParagraph, PortfolioProjectsParagraph, PortfolioResumeParagraph,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentField {
    #[serde(rename = "paragraph--content_timeline")]
    ContentTimelineParagraph(ContentTimelineParagraph),
    #[serde(rename = "paragraph--content_hover_card")]
    ContentProjectParagraph(ContentHoverCardParagraph),
    #[serde(rename = "paragraph--content_text")]
    ContentTextParagraph(ContentTextParagraph),
    #[serde(rename = "paragraph--content_media")]
    ContentMediaParagraph(ContentMediaParagraph),
    #[serde(rename = "paragraph--content_slider")]
    ContentSlider(ContentSliderParagraph),
    #[serde(rename = "paragraph--portfolio_about_me")]
    PortfolioAboutMeParagraph(PortfolioAboutMeParagraph),
    #[serde(rename = "paragraph--portfolio_resume")]
    PortfolioResumeParagraph(PortfolioResumeParagraph),
    #[serde(rename = "paragraph--portfolio_projects")]
    PortfolioProjectsParagraph(PortfolioProjectsParagraph),
    #[serde(rename = "paragraph--portfolio_blog")]
    PortfolioArticlesParagraph(PortfolioArticlesParagraph),
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TaxonomyField {
    // #[serde(rename = "taxonomy_term--tags")]
    // TagsVocabulary(TagsVocabulary),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct PathField {
    alias: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pid: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct LinkField {
    uri: String,
    title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct ImageField {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    name: String,
    media_image: ImageMedia,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct DocumentField {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
    name: String,
    media_document: DocumentMedia,
}

#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
pub struct MetatagsField {
    tag: MetatagTypeField,
    attributes: MetatagAttributesField,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MetatagTypeField {
    Meta,
    Link,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MetatagAttributesField {
    Named { name: String, content: String },
    Property { property: String, content: String },
    Link { rel: String, href: String },
}
