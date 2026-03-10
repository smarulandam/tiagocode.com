//! Shared UI for all platforms.

use dioxus::prelude::*;

pub mod components;
pub mod layouts;

pub use components::common::{NotFoundError, UnexpectedError};
pub use layouts::SiteLayout;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const CUSTOM_CSS: Asset = asset!("/assets/css/custom.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const SPLIDE_CSS: Asset = asset!("/assets/plugins/splidejs/css/splide.min.css");
pub const SPLIDE_JS: Asset = asset!("/assets/plugins/splidejs/js/splide.min.js");
pub const PRISM_TOMORROW_CSS: Asset = asset!("/assets/plugins/prismjs/prism-tomorrow.min.css");
pub const PRISM_TOOLBAR_CSS: Asset = asset!("/assets/plugins/prismjs/prism-toolbar.min.css");
pub const PRISM_CORE_JS: Asset = asset!("/assets/plugins/prismjs/prism-core.min.js");
pub const PRISM_TOOLBAR_JS: Asset = asset!("/assets/plugins/prismjs/prism-toolbar.min.js");
pub const PRISM_COPY_JS: Asset = asset!("/assets/plugins/prismjs/prism-copy-to-clipboard.min.js");
pub const PRISM_AUTOLOADER_JS: Asset = asset!("/assets/plugins/prismjs/prism-autoloader.min.js");
