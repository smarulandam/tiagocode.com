use crate::application::domain::core::{AppError, Result};
use crate::application::domain::portfolio::{BlogsBuilder, Portfolio, PortfolioSection};
use crate::application::ports::driven::{ForFetchingArticlesFeatured, ForFetchingPortfolioData};
use crate::application::ports::driver::ForDisplayingPortfolio;
use crate::application::value_objects::ModerationStatus;
use async_trait::async_trait;

/// Service for retrieving portfolio data
///
/// This service implements the GetPortfolioQuery interface and uses a repository
/// that implements LoadPortfolioPort to fetch portfolio data.
pub struct ShowPortfolioDetailUseCase {
    portfolio_repository: Box<dyn ForFetchingPortfolioData>,
    articles_repository: Box<dyn ForFetchingArticlesFeatured>,
}

impl ShowPortfolioDetailUseCase {
    pub fn new(
        portfolio_repository: Box<dyn ForFetchingPortfolioData>,
        articles_repository: Box<dyn ForFetchingArticlesFeatured>,
    ) -> Self {
        Self {
            portfolio_repository,
            articles_repository,
        }
    }
}

#[async_trait(?Send)]
impl ForDisplayingPortfolio for ShowPortfolioDetailUseCase {
    async fn execute(&self) -> Result<Portfolio> {
        let mut portfolio = self
            .portfolio_repository
            .find_by_slug("/en/portfolio/santiago-marulanda")
            .await?;

        if portfolio.status().eq(&ModerationStatus::Unpublished) {
            return Err(AppError::Forbidden {
                reason: "Unpublished".to_string(),
            });
        }

        let articles = self.articles_repository.get_featured().await?;

        for section in portfolio.sections_mut() {
            if let PortfolioSection::Blogs(b) = section {
                let updated_blog = BlogsBuilder::default()
                    .id(b.id().clone())
                    .title(b.title().clone())
                    .subtitle(b.subtitle().clone())
                    .text(b.text().clone())
                    .articles(articles.to_vec())
                    .build()
                    .unwrap();

                *section = PortfolioSection::Blogs(updated_blog);
            }
        }

        Ok(portfolio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::{Article, Articles};
    use crate::application::domain::portfolio::tests::portfolio_fixture;
    use crate::application::domain::portfolio::tests::unpublished_portfolio_fixture;

    pub struct PortfolioRepositoryMock {
        fixture: Portfolio,
    }

    struct ArticleRepositoryMock {
        fixture: Vec<Article>,
    }

    impl PortfolioRepositoryMock {
        pub fn with_fixture(fixture: Portfolio) -> Self {
            Self { fixture }
        }
    }

    impl ArticleRepositoryMock {
        pub fn with_fixture(fixture: Vec<Article>) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingPortfolioData for PortfolioRepositoryMock {
        async fn find_by_slug(&self, _slug: &str) -> Result<Portfolio> {
            Ok(self.fixture.clone())
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingArticlesFeatured for ArticleRepositoryMock {
        async fn get_featured(&self) -> Result<Articles> {
            Ok(self.fixture.clone())
        }
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_valid_portfolio_is_retrieved() {
        let fixture = portfolio_fixture();
        let portfolio_repo_mock = Box::new(PortfolioRepositoryMock::with_fixture(fixture.clone()));
        let article_repo_mock = Box::new(ArticleRepositoryMock::with_fixture(vec![]));

        let use_case = ShowPortfolioDetailUseCase::new(portfolio_repo_mock, article_repo_mock);
        let fetched_portfolio = use_case.execute().await.unwrap();

        assert_eq!(fetched_portfolio.id(), fixture.id());
        assert_eq!(fetched_portfolio.title(), fixture.title());
        assert_eq!(fetched_portfolio.status(), fixture.status());
        assert_eq!(fetched_portfolio.sections().len(), fixture.sections().len());
    }

    #[actix_rt::test]
    async fn executor_fails_when_unpublished_portfolio_is_retrieved() {
        let fixture = unpublished_portfolio_fixture();
        let portfolio_repo_mock = Box::new(PortfolioRepositoryMock::with_fixture(fixture.clone()));
        let article_repo_mock = Box::new(ArticleRepositoryMock::with_fixture(vec![]));

        let use_case = ShowPortfolioDetailUseCase::new(portfolio_repo_mock, article_repo_mock);
        let fetched_portfolio = use_case.execute().await;

        assert!(matches!(fetched_portfolio, Err(AppError::Forbidden { .. })));
    }
}
