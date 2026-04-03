use lazy_static::lazy_static;
use regex::Regex;
use voca_rs::strip::strip_tags;

use crate::adapters::driven::drupal_jsonapi::entities::{ArticleNode, ContentField, PathField};
use crate::adapters::driven::drupal_jsonapi::entities::{ImageField, TagsVocabulary};
use crate::adapters::driven::drupal_jsonapi::mappers::metatags_field_mapper;
use crate::application::domain::article::{
    Article, ArticleBuilder, ArticleContent, Articles, Category, CategoryBuilder, ContentTable,
    ContentTableItem,
};
use crate::application::domain::common::{Image, ImageBuilder};
use crate::application::domain::core::{AppError, Result};
use crate::application::value_objects::{RequiredText, Url};

lazy_static! {
    static ref BLOCKED_CONTENT_ATTRIBUTES: Regex =
        Regex::new(r#"(style=".*?"|data-\w+=".*?"|data-pm-slice=".*?")"#).unwrap();
    static ref ARTICLE_SECTION_HEADING_REGEX: Regex = Regex::new(
        r#"(?is)<h(?<level>[1-3])(?<attrs>[^>]*)>(?<html>.*?)</h(?<closing_level>[1-3])>"#,
    )
    .unwrap();
    static ref HEADING_ID_ATTRIBUTE_REGEX: Regex =
        Regex::new(r#"(?i)\bid\s*=\s*["']([^"']+)["']"#).unwrap();
}

const GIF_MIME_TYPE: &str = "image/gif";

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
    let content = content_field_mapper(&node);
    let content_table = content_table_mapper(&content);

    ArticleBuilder::default()
        .id(node.id().to_string().try_into()?)
        .slug(slug_field_mapper(node.path())?)
        .status(node.status().clone().into())
        .title(node.title().to_string().try_into()?)
        .summary(node.body().to_string().try_into()?)
        .created_at(node.created_at().to_string().try_into()?)
        .category(tag_vocabulary_mapper(node.tags().clone()))
        .thumbnail(thumbnail_field_mapper(node.thumbnail()))
        .metatags(metatags_field_mapper(node.metatags()))
        .content(content)
        .content_table(content_table)
        .build()
        .map_err(|e| AppError::unexpected(e))
}

pub fn slug_field_mapper(field: &PathField) -> Result<Url> {
    let lang_code = field.langcode();

    if lang_code.ne("en") {
        format!("/{lang_code}{}", field.alias().to_string()).try_into()
    } else {
        field.alias().to_string().try_into()
    }
}

fn content_field_mapper(data: &ArticleNode) -> Vec<ArticleContent> {
    data.content().iter().map(content_elements_mapper).collect()
}

fn content_table_mapper(content: &[ArticleContent]) -> ContentTable {
    let mut table_of_contents_items = Vec::new();

    for content_block in content {
        let ArticleContent::Text(text) = content_block else {
            continue;
        };

        for capture in ARTICLE_SECTION_HEADING_REGEX.captures_iter(text.as_str()) {
            let Some(level_match) = capture.name("level") else {
                continue;
            };

            let Some(closing_level_match) = capture.name("closing_level") else {
                continue;
            };

            if level_match.as_str() != closing_level_match.as_str() {
                continue;
            }

            let Some(attrs_match) = capture.name("attrs") else {
                continue;
            };

            let Some(inner_html_match) = capture.name("html") else {
                continue;
            };

            let title = strip_tags(inner_html_match.as_str())
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            if title.is_empty() {
                continue;
            }

            table_of_contents_items.push(ContentTableItem {
                title,
                level: level_match.as_str().parse::<u8>().unwrap_or(2),
                id: HEADING_ID_ATTRIBUTE_REGEX
                    .captures(attrs_match.as_str())
                    .and_then(|capture| capture.get(1))
                    .map(|value| value.as_str().trim().to_string())
                    .filter(|value| !value.is_empty()),
            });
        }
    }

    table_of_contents_items
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
            p.media_list()
                .iter()
                .map(slider_thumbnail_field_mapper)
                .collect(),
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
    let media = p.media_image();
    let original_url = absolute_asset_url(
        media.uri().url().as_str(),
        media.image_style_uri().max_900x550().as_str(),
    );

    let url = if media.mime_type() == GIF_MIME_TYPE {
        original_url.clone()
    } else {
        media.image_style_uri().max_900x550().to_string()
    }
    .try_into()
    .unwrap();

    let url_high_resolution = if media.mime_type() == GIF_MIME_TYPE {
        original_url
    } else {
        media.image_style_uri().max_2600x2600().to_string()
    }
    .try_into()
    .unwrap();

    ImageBuilder::default()
        .id(p.id().to_string().try_into().unwrap())
        .title(media.meta().alt().to_string().try_into().unwrap())
        .alt(media.meta().alt().to_string().try_into().unwrap())
        .height(media.meta().height().clone())
        .width(media.meta().width().clone())
        .url(url)
        .url_high_resolution(Some(url_high_resolution))
        .build()
        .unwrap()
}

fn thumbnail_field_mapper(p: &ImageField) -> Image {
    let url = p
        .media_image()
        .image_style_uri()
        .thumbnail_800x500()
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
        .url_high_resolution(None::<Url>)
        .build()
        .unwrap()
}

fn slider_thumbnail_field_mapper(p: &ImageField) -> Image {
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
        .url_high_resolution(None::<Url>)
        .build()
        .unwrap()
}

fn absolute_asset_url(asset_path: &str, fallback_absolute_url: &str) -> String {
    if asset_path.starts_with("http://") || asset_path.starts_with("https://") {
        return asset_path.to_string();
    }

    let Some(scheme_end) = fallback_absolute_url.find("://").map(|index| index + 3) else {
        return asset_path.to_string();
    };

    let origin = match fallback_absolute_url[scheme_end..].find('/') {
        Some(path_start) => &fallback_absolute_url[..scheme_end + path_start],
        None => fallback_absolute_url,
    };

    if asset_path.starts_with('/') {
        format!("{origin}{asset_path}")
    } else {
        format!("{origin}/{asset_path}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::ArticleContent;
    use crate::application::value_objects::RequiredText;
    use std::fs;

    fn image_field_fixture(file_name: &str) -> ImageField {
        let fixture_path = format!("{}/tests/fixtures/{file_name}", env!("CARGO_MANIFEST_DIR"));
        let content = fs::read_to_string(fixture_path).unwrap();

        serde_json::from_str(&content).unwrap()
    }

    fn static_image_field_fixture() -> ImageField {
        image_field_fixture("article_mapper_static_image_field.json")
    }

    fn gif_image_field_fixture() -> ImageField {
        image_field_fixture("article_mapper_gif_image_field.json")
    }

    fn text_block(value: &str) -> ArticleContent {
        ArticleContent::Text(RequiredText::try_from(value).unwrap())
    }

    #[test]
    fn article_thumbnail_mapper_uses_800x500_style_without_high_resolution() {
        let image = thumbnail_field_mapper(&static_image_field_fixture());

        assert!(image.url().as_str().contains("/styles/thumbnail_800x500/"));
        assert!(image.url_high_resolution().is_none());
    }

    #[test]
    fn slider_thumbnail_mapper_keeps_260x210_style_without_high_resolution() {
        let image = slider_thumbnail_field_mapper(&static_image_field_fixture());

        assert!(image.url().as_str().contains("/styles/thumbnail_260x210/"));
        assert!(image.url_high_resolution().is_none());
    }

    #[test]
    fn image_mapper_uses_render_and_high_resolution_styles_for_static_images() {
        let image = image_field_mapper(&static_image_field_fixture());

        assert!(image.url().as_str().contains("/styles/max_900x550/"));
        assert!(image
            .url_high_resolution()
            .as_ref()
            .unwrap()
            .as_str()
            .contains("/styles/max_2600x2600/"));
    }

    #[test]
    fn image_mapper_uses_original_absolute_url_for_gifs() {
        let image = image_field_mapper(&gif_image_field_fixture());
        let expected = "https://local-admin.tiagocode.com/sites/default/files/2026-03/dailyfina-hydrangea-26262.gif";

        assert_eq!(image.url().as_str(), expected);
        assert_eq!(
            image.url_high_resolution().as_ref().unwrap().as_str(),
            expected
        );
    }

    #[test]
    fn content_table_mapper_collects_headings_from_text_blocks_without_touching_non_text_blocks() {
        let content = vec![
            ArticleContent::Unknown,
            text_block(
                r#"
                    <h2 id="intro"><span>Hello</span> <strong>World</strong> 🧠</h2>
                    <p>Body</p>
                    <h3>Inner topic</h3>
                    <h4>Ignored</h4>
                "#,
            ),
        ];

        let table_of_contents_items = content_table_mapper(&content);

        assert_eq!(table_of_contents_items.len(), 2);
        assert_eq!(table_of_contents_items[0].title, "Hello World 🧠");
        assert_eq!(table_of_contents_items[0].level, 2);
        assert_eq!(table_of_contents_items[0].id.as_deref(), Some("intro"));
        assert_eq!(table_of_contents_items[1].title, "Inner topic");
        assert_eq!(table_of_contents_items[1].level, 3);
        assert_eq!(table_of_contents_items[1].id, None);
    }

    #[test]
    fn content_table_mapper_keeps_headings_without_ids_visible() {
        let content = vec![text_block(
            "<h2>Overview</h2><h3 id=\"details\">Details</h3>",
        )];

        let table_of_contents_items = content_table_mapper(&content);

        assert_eq!(table_of_contents_items.len(), 2);
        assert_eq!(table_of_contents_items[0].title, "Overview");
        assert_eq!(table_of_contents_items[0].id, None);
        assert_eq!(table_of_contents_items[1].title, "Details");
        assert_eq!(table_of_contents_items[1].id.as_deref(), Some("details"));
    }

    #[test]
    fn content_table_mapper_preserves_heading_order_across_text_blocks_and_ignores_h4() {
        let content = vec![
            text_block("<h2 id=\"intro\">Intro</h2><h4>Ignore me</h4>"),
            text_block("<h3>Second step</h3><h1 id=\"final\">Final</h1>"),
        ];

        let table_of_contents_items = content_table_mapper(&content);

        assert_eq!(table_of_contents_items.len(), 3);
        assert_eq!(table_of_contents_items[0].title, "Intro");
        assert_eq!(table_of_contents_items[1].title, "Second step");
        assert_eq!(table_of_contents_items[2].title, "Final");
        assert_eq!(table_of_contents_items[2].level, 1);
        assert_eq!(table_of_contents_items[2].id.as_deref(), Some("final"));
    }
}
