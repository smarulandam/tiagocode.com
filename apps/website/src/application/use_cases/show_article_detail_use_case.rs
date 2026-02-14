use async_trait::async_trait;

use crate::application::domain::article::Article;
use crate::application::domain::core::{AppError, Result};
use crate::application::ports::driven::ForFetchingArticleData;
use crate::application::ports::driver::ForDisplayingArticle;
use crate::application::value_objects::ModerationStatus;

/// Service for retrieving article data
///
/// This use case implements the ForDisplayingArticle interface and uses a repository
/// that implements ForFetchingArticleData to fetch the necessary data for displaying
/// an article.
pub struct ShowArticleDetailUseCase {
    article_repository: Box<dyn ForFetchingArticleData>,
}

impl ShowArticleDetailUseCase {
    pub fn new(article_repository: Box<dyn ForFetchingArticleData>) -> Self {
        Self { article_repository }
    }
}

#[async_trait(?Send)]
impl ForDisplayingArticle for ShowArticleDetailUseCase {
    async fn execute(&self, slug: &str) -> Result<Article> {
        let article = self.article_repository.find_by_slug(slug).await?;

        if article.status().eq(&ModerationStatus::Unpublished) {
            return Err(AppError::Forbidden {
                reason: "Unpublished".to_string(),
            });
        }

        Ok(article)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::domain::article::tests::article_fixture;
    use crate::application::domain::article::tests::unpublished_article_fixture;
    use crate::application::domain::article::Article;

    struct ArticleRepositoryMock {
        fixture: Article,
    }

    impl ArticleRepositoryMock {
        pub fn with_fixture(fixture: Article) -> Self {
            Self { fixture }
        }
    }

    #[async_trait(?Send)]
    impl ForFetchingArticleData for ArticleRepositoryMock {
        async fn find_by_slug(&self, _slug: &str) -> Result<Article> {
            Ok(self.fixture.clone())
        }
    }

    #[actix_rt::test]
    async fn executor_succeeds_when_valid_article_is_retrieved() {
        let article_fixture = article_fixture();
        let article_repository = ArticleRepositoryMock::with_fixture(article_fixture.clone());

        let use_case = ShowArticleDetailUseCase::new(Box::new(article_repository));
        let result = use_case
            .execute(article_fixture.slug().as_str())
            .await
            .unwrap();

        assert_eq!(result.id(), article_fixture.id());
        assert_eq!(result.title(), article_fixture.title());
        assert_eq!(result.slug(), article_fixture.slug());
        assert_eq!(result.summary(), article_fixture.summary());
        assert_eq!(result.status(), article_fixture.status());
        assert_eq!(result.created_at(), article_fixture.created_at());
        assert_eq!(result.content().len(), article_fixture.content().len());
    }

    #[actix_rt::test]
    async fn executor_fails_when_unpublished_article_is_retrieved() {
        let article_fixture = unpublished_article_fixture();
        let article_repository = ArticleRepositoryMock::with_fixture(article_fixture.clone());

        let use_case = ShowArticleDetailUseCase::new(Box::new(article_repository));
        let result = use_case.execute(article_fixture.slug().as_str()).await;

        assert!(matches!(result, Err(AppError::Forbidden { .. })));
    }
}
