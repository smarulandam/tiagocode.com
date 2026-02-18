use async_trait::async_trait;

use crate::application::domain::core::{AppError, Result};
use crate::application::domain::layout::{Layout, LayoutBuilder};
use crate::application::ports::driven::ForFetchingMenuData;
use crate::application::ports::driver::ForDisplayingLayout;

/// Service for retrieving layout data
///
/// This service implements the GetLayoutQuery interface and uses a repository
/// that implements LoadMenuPort to fetch menu data for constructing layouts.
///
pub struct GetLayoutUseCase {
    repository: Box<dyn ForFetchingMenuData>,
}

impl GetLayoutUseCase {
    /// Creates a new GetLayoutUseCase with the given repository
    ///
    /// # Arguments
    /// * `repository` - A boxed trait object implementing LoadMenuPort
    ///
    pub fn new(repository: Box<dyn ForFetchingMenuData>) -> Self {
        Self { repository }
    }
}

#[async_trait(?Send)]
impl ForDisplayingLayout for GetLayoutUseCase {
    async fn execute(&self) -> Result<Layout> {
        let main_menu = self.repository.find_by_id("main").await?;
        let social_menu = self.repository.find_by_id("social-network").await?;

        let layout = LayoutBuilder::default()
            .logo(None)
            .main_menu(main_menu)
            .social_menu(social_menu)
            .sidebar_menu(None)
            .footer_menu(None)
            .build()
            .map_err(AppError::unexpected)?;

        Ok(layout)
    }
}
