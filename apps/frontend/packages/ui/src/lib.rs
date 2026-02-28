//! Shared UI for all platforms.

use dioxus::prelude::*;

pub mod components;
pub mod layouts;
pub mod views;

pub use components::common::{NotFoundError, UnexpectedError};
pub use layouts::AppShell;
pub use views::{ArticleDetailView, ArticlesListView, NotFoundView, PortfolioView};

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const CUSTOM_CSS: Asset = asset!("/assets/css/custom.css");
