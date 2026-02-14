use async_trait::async_trait;

use crate::application::domain::article::Article;
use crate::application::domain::core::Result;

/// Output port for retrieving the article data.
///
/// This trait defines the contract for fetching article entity based on different parameters.
/// Implementations of this trait should handle data access, whether from a database, API,
/// or other storage mechanisms.
#[async_trait(?Send)]
pub trait ForFetchingArticleData {
    /// Retrieves the article from the data source.
    async fn find_by_slug(&self, slug: &str) -> Result<Article>;
}
