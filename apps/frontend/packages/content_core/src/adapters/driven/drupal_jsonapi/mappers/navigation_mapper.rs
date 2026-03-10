use itertools::Itertools;
use uuid::Uuid;

use crate::adapters::driven::drupal_jsonapi::entities::NavigationImageMetadata;
use crate::adapters::driven::drupal_jsonapi::entities::{Navigation, NavigationItem};
use crate::application::domain::common::{Image, ImageBuilder};
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::layout::{MenuItem, MenuItemBuilder, MenuTree, MenuTreeBuilder};

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
        item.relative().to_string()
    }
}

fn external_image_mapper(image: &NavigationImageMetadata) -> Image {
    ImageBuilder::default()
        .id(Uuid::new_v4().to_string().try_into().unwrap())
        .url(image.url().to_string().try_into().unwrap())
        .alt(image.alt().to_string().try_into().unwrap())
        .title(image.alt().to_string().try_into().unwrap())
        .width(image.width().clone())
        .height(image.height().clone())
        .build()
        .unwrap()
}
