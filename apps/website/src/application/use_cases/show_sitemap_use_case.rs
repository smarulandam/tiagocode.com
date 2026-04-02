use async_trait::async_trait;

use crate::application::domain::core::Result;
use crate::application::domain::sitemap::Sitemap;
use crate::application::ports::driven::ForFetchingSitemapEntries;
use crate::application::ports::driver::ForDisplayingSitemap;

pub struct ShowSitemapUseCase {
    sitemap_repository: Box<dyn ForFetchingSitemapEntries>,
}

impl ShowSitemapUseCase {
    pub fn new(sitemap_repository: Box<dyn ForFetchingSitemapEntries>) -> Self {
        Self { sitemap_repository }
    }
}

#[async_trait(?Send)]
impl ForDisplayingSitemap for ShowSitemapUseCase {
    async fn execute(&self) -> Result<Sitemap> {
        self.sitemap_repository.find_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::sitemap::tests::sitemap_fixture;
    use crate::application::ports::driven::ForFetchingSitemapEntries;

    struct SitemapRepositoryMock {
        fixture: Sitemap,
    }

    impl SitemapRepositoryMock {
        fn with_fixture(fixture: Sitemap) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingSitemapEntries for SitemapRepositoryMock {
        async fn find_all(&self) -> Result<Sitemap> {
            Ok(self.fixture.clone())
        }
    }

    #[actix_rt::test]
    async fn executor_returns_repository_entries_without_extra_transformations() {
        let repository = SitemapRepositoryMock::with_fixture(sitemap_fixture());
        let use_case = ShowSitemapUseCase::new(Box::new(repository));
        let result = use_case.execute().await.unwrap();

        assert_eq!(result.entries().len(), 1);
        assert_eq!(
            result.entries()[0].location().as_str(),
            "/en/articles/rust/what-is-ownership"
        );
        assert_eq!(
            result.entries()[0]
                .last_modification()
                .as_ref()
                .unwrap()
                .to_rfc3339(),
            "2026-04-02T08:15:00+00:00"
        );
    }
}
