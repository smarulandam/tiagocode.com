use async_trait::async_trait;

use crate::application::domain::core::Result;
use crate::application::domain::portfolio::Portfolio;

/// Output port for retrieving portfolio data.
///
/// This trait defines the contract for fetching portfolio entities based on their slug.
/// Implementations of this trait should handle data access, whether from a database, API,
/// or other storage mechanisms.
#[async_trait(?Send)]
pub trait ForFetchingPortfolioData {
    /// Retrieves a portfolio by its slug.
    ///
    /// # Arguments
    /// * `slug` - A string slice that holds the slug of the portfolio to be retrieved.
    ///
    /// # Returns
    /// * `AppError<Portfolio>` - A result wrapping the `Portfolio` if found, or an error
    ///   if the portfolio could not be retrieved.
    async fn find_by_slug(&self, slug: &str) -> Result<Portfolio>;
}
