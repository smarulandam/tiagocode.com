use async_trait::async_trait;

use crate::application::domain::article::Article;
use crate::application::domain::core::Result;

/// Use case for displaying article detail.
///
/// This input port defines the application logic for fetching the article details.
/// It acts as a boundary between the web UI (controller) and the application core,
/// allowing the web UI to obtain the article data for rendering.
#[async_trait(?Send)]
pub trait ForDisplayingArticle {
    /// Retrieves the article full information.
    ///
    /// # Returns
    /// * `AppError<Article>` - A result wrapping the `Articles` if found,
    /// or an error if the article could not be retrieved.
    async fn execute(&self, slug: &str) -> Result<Article>;
}
