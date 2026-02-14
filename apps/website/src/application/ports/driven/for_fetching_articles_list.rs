use async_trait::async_trait;

use crate::application::domain::article::Article;
use crate::application::domain::core::Result;

/// Output port for retrieving list of articles data.
///
/// This trait defines the contract for fetching articles entities based on different parameters.
/// Implementations of this trait should handle data access, whether from a database, API,
/// or other storage mechanisms.
#[async_trait(?Send)]
pub trait ForFetchingArticlesList {
    /// Retrieves all the articles from the data source.
    ///
    /// Returns a collection of articles sorted by recency, typically used
    /// for displaying latest content in feeds or home pages.
    async fn get_list(&self, category_id: Option<String>) -> Result<Vec<Article>>;
}
