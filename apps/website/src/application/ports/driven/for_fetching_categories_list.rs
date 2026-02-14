use async_trait::async_trait;

use crate::application::domain::article::Category;
use crate::application::domain::core::Result;

/// Output port for retrieving list of categories data.
///
/// This trait defines the contract for fetching category entities.
/// Implementations of this trait should handle data access, whether from a database, API,
/// or other storage mechanisms.
#[async_trait(?Send)]
pub trait ForFetchingCategoriesList {
    /// Retrieves all categories from the data source.
    ///
    /// Returns a collection of categories, typically used
    /// for displaying category filters or navigation menus.
    async fn find_all_categories(&self) -> Result<Vec<Category>>;
}
