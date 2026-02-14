use lazy_static::lazy_static;
use regex::Regex;

use crate::adapters::driven::drupal_jsonapi::entities::{ArticleNode, ContentField};
use crate::adapters::driven::drupal_jsonapi::entities::{ImageField, TagsVocabulary};
use crate::adapters::driven::drupal_jsonapi::mappers::metatags_field_mapper;
use crate::application::domain::article::{Article, ArticleBuilder, Articles};
use crate::application::domain::article::{ArticleContent, Category, CategoryBuilder};
use crate::application::domain::common::{Image, ImageBuilder};
use crate::application::domain::core::{AppError, Result};
use crate::application::value_objects::RequiredText;

lazy_static! {
    static ref BLOCKED_CONTENT_ATTRIBUTES: Regex =
        Regex::new(r#"(style=".*?"|data-\w+=".*?"|data-pm-slice=".*?")"#).unwrap();
}

/// Trait for converting external data into an `Article` domain entity.
/// Ensures separation between external data sources and core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type to be transformed into an `Article`.
pub trait ExternalArticleMapper {
    type Input;

    /// Converts external data into an `Article`.
    ///
    /// # Arguments
    /// * `input` - The external data to be transformed into an `Article`.
    ///
    /// # Returns
    /// * `Result<Article>` - The result of the transformation.
    fn adapt(&self, input: Self::Input) -> Result<Article>;

    fn adapt_multiple(&self, input: Vec<Self::Input>) -> Result<Articles> {
        input
            .into_iter()
            .map(|article| self.adapt(article))
            .collect()
    }
}

#[derive(Default)]
pub struct ArticleNodeMapper;

impl ExternalArticleMapper for ArticleNodeMapper {
    type Input = ArticleNode;

    fn adapt(&self, input: Self::Input) -> Result<Article> {
        article_node_mapper(input)
    }
}

fn article_node_mapper(node: ArticleNode) -> Result<Article> {
    ArticleBuilder::default()
        .id(node.id().to_string().try_into()?)
        .slug(node.path().alias().to_string().try_into()?)
        .status(node.status().clone().into())
        .title(node.title().to_string().try_into()?)
        .summary(node.body().to_string().try_into()?)
        .created_at(node.created_at().to_string().try_into()?)
        .category(tag_vocabulary_mapper(node.tags().clone()))
        .thumbnail(thumbnail_field_mapper(node.thumbnail()))
        .metatags(metatags_field_mapper(node.metatags()))
        .content(content_field_mapper(node))
        .build()
        .map_err(|e| AppError::unexpected(e))
}

fn content_field_mapper(data: ArticleNode) -> Vec<ArticleContent> {
    data.content().iter().map(content_elements_mapper).collect()
}

fn content_elements_mapper(content: &ContentField) -> ArticleContent {
    match content {
        ContentField::ContentMediaParagraph(_) => media_paragraph_adapter(&content),
        ContentField::ContentTextParagraph(_) => text_paragraph_mapper(&content),
        ContentField::ContentSlider(_) => slider_paragraph_adapter(&content),
        _ => ArticleContent::Unknown,
    }
}

fn media_paragraph_adapter(p: &ContentField) -> ArticleContent {
    if let ContentField::ContentMediaParagraph(p) = p {
        return ArticleContent::Image(image_field_mapper(p.media()));
    }
    ArticleContent::Unknown
}

fn slider_paragraph_adapter(p: &ContentField) -> ArticleContent {
    if let ContentField::ContentSlider(p) = p {
        return ArticleContent::Slider(
            p.media_list().iter().map(thumbnail_field_mapper).collect(),
            p.media_list().iter().map(image_field_mapper).collect(),
        );
    }

    ArticleContent::Unknown
}

fn text_paragraph_mapper(p: &ContentField) -> ArticleContent {
    if let ContentField::ContentTextParagraph(p) = p {
        let content: RequiredText = BLOCKED_CONTENT_ATTRIBUTES
            .replace_all(p.text().as_str(), "")
            .to_string()
            .try_into()
            .unwrap();

        return ArticleContent::Text(content);
    }
    ArticleContent::Unknown
}

fn tag_vocabulary_mapper(tag: TagsVocabulary) -> Category {
    CategoryBuilder::default()
        .id(tag.id().to_string().try_into().unwrap())
        .slug(tag.path().alias().to_string().try_into().unwrap())
        .title(tag.name().to_string().try_into().unwrap())
        .status(tag.status().clone().into())
        .emoji(tag.emoji().to_string().try_into().unwrap())
        .build()
        .unwrap()
}

fn image_field_mapper(p: &ImageField) -> Image {
    let url = p
        .media_image()
        .image_style_uri()
        .max_900x550()
        .to_string()
        .try_into()
        .unwrap();

    ImageBuilder::default()
        .id(p.id().to_string().try_into().unwrap())
        .title(p.media_image().meta().alt().to_string().try_into().unwrap())
        .alt(p.media_image().meta().alt().to_string().try_into().unwrap())
        .height(p.media_image().meta().height().clone())
        .width(p.media_image().meta().width().clone())
        .url(url)
        .build()
        .unwrap()
}

fn thumbnail_field_mapper(p: &ImageField) -> Image {
    let url = p
        .media_image()
        .image_style_uri()
        .thumbnail_260x210()
        .to_string()
        .try_into()
        .unwrap();

    ImageBuilder::default()
        .id(p.id().to_string().try_into().unwrap())
        .title(p.media_image().meta().alt().to_string().try_into().unwrap())
        .alt(p.media_image().meta().alt().to_string().try_into().unwrap())
        .height(p.media_image().meta().height().clone())
        .width(p.media_image().meta().width().clone())
        .url(url)
        .build()
        .unwrap()
}
