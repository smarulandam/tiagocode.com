use async_trait::async_trait;

use crate::application::domain::core::Result;
use crate::application::domain::portfolio::Portfolio;

/// Input port for retrieving portfolio data.
///
/// It defines the application logic for fetching a portfolio, and
/// acts as a boundary between the web UI (controller) and the application core,
/// allowing the web UI to obtain portfolio data for rendering.
#[async_trait(?Send)]
pub trait ForDisplayingPortfolio {
    /// Retrieves a portfolio.
    ///
    /// # Returns
    /// * `AppError<Portfolio>` - A result wrapping the `Portfolio` if found, or an error
    ///   if the portfolio could not be retrieved.
    async fn execute(&self) -> Result<Portfolio>;
}
