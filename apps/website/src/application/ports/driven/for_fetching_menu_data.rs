use async_trait::async_trait;

use crate::application::domain::core::Result;
use crate::application::domain::layout::MenuTree;

/// Output port for retrieving menu data.
///
/// This trait defines the contract for fetching menu tree structures based on their ID.
/// Implementations of this trait should handle data access from external sources such as
/// databases, APIs, or other storage mechanisms.
#[async_trait(?Send)]
pub trait ForFetchingMenuData {
    /// Finds a menu tree by its ID.
    ///
    /// # Arguments
    /// * `id` - The identifier for the menu to retrieve
    ///
    /// # Returns
    /// - `AppError<MenuTree>`: The menu tree if found, or an error if retrieval fails
    async fn find_by_id(&self, id: &str) -> Result<MenuTree>;
}
