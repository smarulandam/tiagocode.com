use async_trait::async_trait;

use crate::application::domain::core::Result;
use crate::application::domain::sitemap::Sitemap;

#[async_trait(?Send)]
pub trait ForFetchingSitemapEntries {
    async fn find_all(&self) -> Result<Sitemap>;
}
