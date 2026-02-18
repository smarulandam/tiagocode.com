use async_trait::async_trait;

use crate::application::domain::article::{Article, Category};
use crate::application::domain::core::Result;
use crate::application::domain::page::Page;

/// Use case for listing the articles.
///
/// This input port defines the application logic for fetching all articles.
/// It acts as a boundary between the web UI (controller) and the application core,
/// allowing the web UI to obtain articles data for rendering.
#[async_trait(?Send)]
pub trait ForDisplayingArticlesList {
    /// Retrieves a list of articles and categories, optionally filtered by category.
    ///
    /// # Arguments
    /// * `category_id` - Optional category identifier to filter articles by category
    ///
    /// # Returns
    /// * `Result<(Vec<Category>, Vec<Article>)>` - A result containing a tuple of:
    ///   - All available categories
    ///   - Articles (filtered by the specified category if provided)
    ///
    /// # Errors
    /// Returns an error if the articles or categories could not be retrieved.
    async fn execute(
        &self,
        slug: &str,
        category_id: Option<String>,
    ) -> Result<(Page, Vec<Category>, Vec<Article>)>;
}
