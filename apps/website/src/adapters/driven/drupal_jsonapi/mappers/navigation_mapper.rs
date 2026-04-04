use itertools::Itertools;
use uuid::Uuid;

use crate::adapters::driven::drupal_jsonapi::entities::NavigationImageMetadata;
use crate::adapters::driven::drupal_jsonapi::entities::{Navigation, NavigationItem};
use crate::application::domain::common::{Image, ImageBuilder};
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::layout::{MenuItem, MenuItemBuilder, MenuTree, MenuTreeBuilder};
use crate::application::value_objects::Url;

/// Trait for converting external data into a `MenuTree` domain entity.
/// Ensures separation between external data sources and core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type to be transformed into a `MenuTree`.
pub trait ExternalMenuTreeMapper {
    type Input;

    /// Converts external data into a `MenuTree`.
    ///
    /// # Arguments
    /// * `input` - The external data to be transformed into a `MenuTree`.
    ///
    /// # Returns
    /// * `ApplicationResult<MenuTree>` - The result of the transformation.
    fn adapt(&self, input: Self::Input) -> Result<MenuTree>;
}

#[derive(Default)]
pub struct NavigationAdapter;

impl ExternalMenuTreeMapper for NavigationAdapter {
    type Input = Navigation;

    fn adapt(&self, input: Self::Input) -> Result<MenuTree> {
        Ok(external_menu_tree_mapper(input)?)
    }
}

fn external_menu_tree_mapper(menu_items: Navigation) -> Result<MenuTree> {
    MenuTreeBuilder::default()
        .items(external_menu_items_mapper(menu_items))
        .build()
        .map_err(|e| AppError::unexpected(e))
}

fn external_menu_items_mapper(menu_items: Navigation) -> Vec<MenuItem> {
    menu_items
        .iter()
        .filter(|item| item.enabled().clone())
        .sorted_by(|a, b| a.weight().cmp(b.weight()))
        .map(external_menu_item_mapper)
        .collect::<Vec<_>>()
        .into_iter()
        .collect()
}

fn external_menu_item_mapper(item: &NavigationItem) -> MenuItem {
    MenuItemBuilder::default()
        .id(Uuid::new_v4().to_string().try_into().unwrap())
        .title(item.title().to_string())
        .url(external_url_mapper(item).try_into().unwrap())
        .hidden(!item.enabled().clone())
        .weight(item.weight().clone())
        .icon(external_icon_mapper(item))
        .build()
        .unwrap()
}

fn external_icon_mapper(item: &NavigationItem) -> Option<Image> {
    item.field_image()
        .as_ref()
        .and_then(|field_image| field_image.field_media_image().first())
        .map(external_image_mapper)
}

fn external_url_mapper(item: &NavigationItem) -> String {
    if item.external().clone() {
        item.absolute().to_string()
    } else {
        normalize_internal_url(item.relative())
    }
}

fn normalize_internal_url(url: &str) -> String {
    if !url.starts_with('/') {
        return url.to_string();
    }

    let suffix_index = url.find(['?', '#']).unwrap_or(url.len());
    let (path, suffix) = url.split_at(suffix_index);
    let mut segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    if segments.len() >= 2
        && segments[0] == segments[1]
        && is_language_segment(segments[0])
    {
        segments.remove(1);
    }

    if segments.first() == Some(&"en") {
        segments.remove(0);
    }

    let mut normalized = if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    };

    normalized.push_str(suffix);
    normalized
}

fn is_language_segment(segment: &str) -> bool {
    segment.len() == 2 && segment.chars().all(|char| char.is_ascii_lowercase())
}

fn external_image_mapper(image: &NavigationImageMetadata) -> Image {
    ImageBuilder::default()
        .id(Uuid::new_v4().to_string().try_into().unwrap())
        .url(image.url().to_string().try_into().unwrap())
        .url_high_resolution(None::<Url>)
        .alt(image.alt().to_string().try_into().unwrap())
        .title(image.alt().to_string().try_into().unwrap())
        .width(image.width().clone())
        .height(image.height().clone())
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::normalize_internal_url;

    #[test]
    fn normalize_internal_url_collapses_duplicated_language_root() {
        assert_eq!(normalize_internal_url("/es/es"), "/es");
        assert_eq!(normalize_internal_url("/en/en"), "/");
    }

    #[test]
    fn normalize_internal_url_collapses_duplicated_language_prefix() {
        assert_eq!(normalize_internal_url("/es/es/articulos"), "/es/articulos");
        assert_eq!(normalize_internal_url("/en/en/articles/test"), "/articles/test");
    }

    #[test]
    fn normalize_internal_url_preserves_query_and_hash() {
        assert_eq!(normalize_internal_url("/es/es?menu=1"), "/es?menu=1");
        assert_eq!(normalize_internal_url("/es/es/articulos#top"), "/es/articulos#top");
        assert_eq!(normalize_internal_url("/en?menu=1"), "/?menu=1");
        assert_eq!(normalize_internal_url("/en/articles#top"), "/articles#top");
    }

    #[test]
    fn normalize_internal_url_remaps_default_language_prefix() {
        assert_eq!(normalize_internal_url("/en"), "/");
        assert_eq!(normalize_internal_url("/en/articles"), "/articles");
    }

    #[test]
    fn normalize_internal_url_leaves_valid_urls_untouched() {
        assert_eq!(normalize_internal_url("/es"), "/es");
        assert_eq!(normalize_internal_url("/es/articulos"), "/es/articulos");
        assert_eq!(normalize_internal_url("/articles"), "/articles");
        assert_eq!(normalize_internal_url("https://example.com"), "https://example.com");
    }
}
