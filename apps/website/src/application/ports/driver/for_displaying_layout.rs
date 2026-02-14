use async_trait::async_trait;

use crate::application::domain::core::Result;
use crate::application::domain::layout::Layout;

/// Use case for retrieving layout (menus, icon, etc.) data.
///
/// It defines the application logic for fetching an article, and
/// acts as a boundary between the web UI (controller) and the application core,
/// allowing the web UI to obtain all the data for rendering.
#[async_trait(?Send)]
pub trait ForDisplayingLayout {
    /// Executes the use case to retrieve layout data.
    ///
    /// # Returns
    /// * `ApplicationResult<Layout>` - A result wrapping the `Layout` if successful, or an error
    ///   if the layout data could not be retrieved.
    async fn execute(&self) -> Result<Layout>;
}
