use async_trait::async_trait;

use crate::application::domain::article::Articles;
use crate::application::domain::core::Result;

/// Output port for retrieving articles data.
///
/// This trait defines the contract for fetching articles entities based on different parameters.
/// Implementations of this trait should handle data access, whether from a database, API,
/// or other storage mechanisms.
#[async_trait(?Send)]
pub trait ForFetchingArticlesFeatured {
    /// Retrieves the most recent articles from the data source.
    ///
    /// Returns a collection of articles sorted by recency, typically used
    /// for displaying latest content in feeds or home pages.
    async fn get_featured(&self) -> Result<Articles>;
}
