use crate::adapters::driven::drupal_jsonapi::entities::PageNode;
use crate::adapters::driven::drupal_jsonapi::mappers::metatags_field_mapper;
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::page::{Page, PageBuilder};

/// Trait for converting external data into a `Portfolio` domain entity.
/// Ensures separation between external data sources and core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type to be transformed into a `Portfolio`.
pub trait ExternalPageAdapter {
    type Input;

    /// Converts external data into a `Portfolio`.
    ///
    /// # Arguments
    /// * `input` - The external data to be transformed into a `Portfolio`.
    ///
    /// # Returns
    /// * `ApplicationResult<Portfolio>` - The result of the transformation.
    fn adapt(&self, input: Self::Input) -> Result<Page>;
}

#[derive(Default)]
pub struct PageNodeMapper;

impl ExternalPageAdapter for PageNodeMapper {
    type Input = PageNode;

    fn adapt(&self, input: Self::Input) -> Result<Page> {
        Ok(page_node_mapper(input)?)
    }
}

fn page_node_mapper(node: PageNode) -> Result<Page> {
    PageBuilder::default()
        .id(node.id().to_string().try_into()?)
        .status(node.status().clone().into())
        .title(node.title().to_string().try_into()?)
        .created_at(node.created_at().to_string().try_into()?)
        .metatags(metatags_field_mapper(node.metatags()))
        .build()
        .map_err(|e| AppError::unexpected(e))
}
