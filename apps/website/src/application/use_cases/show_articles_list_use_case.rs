use async_trait::async_trait;

use crate::application::domain::article::{Article, Category};
use crate::application::domain::core::Result;
use crate::application::domain::page::Page;
use crate::application::ports::driven::{
    ForFetchingArticlesList, ForFetchingCategoriesList, ForFetchingPageData,
};
use crate::application::ports::driver::ForDisplayingArticlesList;
use crate::application::value_objects::ModerationStatus;

/// Service for retrieving articles and categories data
///
/// This use case implements the ForDisplayingArticles interface and uses repositories
/// that implement ForFetchingArticlesList and ForFetchingCategoriesList to fetch
/// the necessary data for displaying articles with their respective categories.
pub struct ShowArticlesListUseCase {
    article_repository: Box<dyn ForFetchingArticlesList>,
    category_repository: Box<dyn ForFetchingCategoriesList>,
    page_repository: Box<dyn ForFetchingPageData>,
}

impl ShowArticlesListUseCase {
    pub fn new(
        article_repository: Box<dyn ForFetchingArticlesList>,
        category_repository: Box<dyn ForFetchingCategoriesList>,
        page_repository: Box<dyn ForFetchingPageData>,
    ) -> Self {
        Self {
            article_repository,
            category_repository,
            page_repository,
        }
    }
}

#[async_trait(?Send)]
impl ForDisplayingArticlesList for ShowArticlesListUseCase {
    async fn execute(
        &self,
        slug: &str,
        category_id: Option<String>,
    ) -> Result<(Page, Vec<Category>, Vec<Article>)> {
        let page = self.page_repository.find_by_slug(slug).await?;

        let articles = self
            .article_repository
            .get_list(category_id)
            .await?
            .into_iter()
            .filter(|a| a.status().eq(&ModerationStatus::Published))
            .collect();

        let categories = self.category_repository.find_all_categories().await?;

        Ok((page, categories, articles))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::tests::{
        article_fixture, category_fixture, unpublished_article_fixture,
    };
    use crate::application::domain::article::{Article, Category};
    use crate::application::domain::page::tests::page_fixture;
    use crate::application::domain::page::{self, Page};

    struct PageRepositoryMock {
        fixture: Page,
    }

    struct ArticleRepositoryMock {
        fixture: Vec<Article>,
    }

    struct CategoryRepositoryMock {
        fixture: Vec<Category>,
    }

    impl PageRepositoryMock {
        pub fn with_fixture(fixture: Page) -> Self {
            Self { fixture }
        }
    }

    impl ArticleRepositoryMock {
        pub fn with_fixture(fixture: Vec<Article>) -> Self {
            Self { fixture }
        }
    }

    impl CategoryRepositoryMock {
        pub fn with_fixture(fixture: Vec<Category>) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingPageData for PageRepositoryMock {
        async fn find_by_slug(&self, slug: &str) -> Result<Page> {
            Ok(self.fixture.clone())
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingArticlesList for ArticleRepositoryMock {
        async fn get_list(&self, _category_id: Option<String>) -> Result<Vec<Article>> {
            Ok(self.fixture.clone())
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingCategoriesList for CategoryRepositoryMock {
        async fn find_all_categories(&self) -> Result<Vec<Category>> {
            Ok(self.fixture.clone())
        }
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_valid_articles_and_categories_are_retrieved() {
        let page_fixture = page_fixture();
        let articles_fixture = vec![article_fixture(), article_fixture()];
        let categories_fixture = vec![category_fixture()];
        let page_repo_mock = PageRepositoryMock::with_fixture(page_fixture.clone());
        let article_repo_mock = ArticleRepositoryMock::with_fixture(articles_fixture.clone());
        let category_repo_mock = CategoryRepositoryMock::with_fixture(categories_fixture.clone());

        let use_case = ShowArticlesListUseCase::new(
            Box::new(article_repo_mock),
            Box::new(category_repo_mock),
            Box::new(page_repo_mock),
        );
        let (fetched_page, fetched_categories, fetched_articles) =
            use_case.execute("/articles", None).await.unwrap();

        assert_eq!(fetched_page.title(), page_fixture.title());
        assert_eq!(fetched_articles.len(), articles_fixture.len());
        assert_eq!(fetched_categories.len(), categories_fixture.len());
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_excluding_unpublished_articles() {
        let page_fixture = page_fixture();
        let articles_fixture = vec![article_fixture(), unpublished_article_fixture()];
        let categories_fixture = vec![category_fixture()];
        let page_repo_mock = PageRepositoryMock::with_fixture(page_fixture.clone());
        let article_repo_mock = ArticleRepositoryMock::with_fixture(articles_fixture.clone());
        let category_repo_mock = CategoryRepositoryMock::with_fixture(categories_fixture.clone());

        let use_case = ShowArticlesListUseCase::new(
            Box::new(article_repo_mock),
            Box::new(category_repo_mock),
            Box::new(page_repo_mock),
        );
        let (_fetched_page, fetched_categories, fetched_articles) =
            use_case.execute("/articles", None).await.unwrap();

        assert_eq!(fetched_articles.len(), 1);
        assert_eq!(fetched_categories.len(), categories_fixture.len());
    }
}
