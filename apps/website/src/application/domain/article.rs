use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::common::{Image, MetaTags};
use crate::application::value_objects::{Date, Identifier, ModerationStatus, RequiredText, Url};

pub type Articles = Vec<Article>;

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Article {
    id: Identifier,
    slug: Url,
    title: RequiredText,
    summary: RequiredText,
    status: ModerationStatus,
    created_at: Date,
    thumbnail: Image,
    content: Vec<ArticleContent>,
    category: Category,
    metatags: MetaTags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArticleContent {
    Image(Image),
    Slider(Vec<Image>, Vec<Image>),
    Text(RequiredText),
    Unknown,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Category {
    id: Identifier,
    slug: Url,
    title: RequiredText,
    emoji: RequiredText,
    status: ModerationStatus,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::application::domain::common::tests::image_fixture;
    use crate::application::domain::common::tests::metatags_fixture;

    #[test]
    fn creation_succeeds_when_valid_article() {
        article_fixture(); // the fixture calls the builder methods, so we just ensure that it doesn't panic
    }

    #[test]
    #[should_panic]
    fn creation_fails_when_invalid_article() {
        invalid_article_fixture();
    }

    #[test]
    fn serialization_succeeds_when_valid_article() {
        let a = article_fixture();
        let serialized = serde_json::json!(&a).to_string();

        assert!(serialized.contains(a.id().to_string().as_str()));
        assert!(serialized.contains(a.slug().as_str()));
        assert!(serialized.contains(a.summary().as_str()));
        assert!(serialized.contains(a.title().as_str()));
        assert!(serialized.contains(a.status().to_string().as_str()));
        assert!(serialized.contains(a.thumbnail().url().as_str()));
        assert!(serialized.contains(a.thumbnail().alt().as_str()));
        assert!(serialized.contains(a.thumbnail().title().as_str()));
        assert!(serialized.contains(a.thumbnail().width().to_string().as_str()));
        assert!(serialized.contains(a.thumbnail().height().to_string().as_str()));
        assert!(serialized.contains(a.category().title().as_str()));
        assert!(serialized.contains(a.category().slug().as_str()));
        assert!(serialized.contains(a.category().emoji().as_str()));
    }

    #[test]
    fn deserialization_succeeds_when_valid_article() {
        let a = article_fixture();
        let serialized = serde_json::json!(&a).to_string();
        let deserialized: Article = serde_json::from_str(&serialized).unwrap();

        assert_eq!(a.id(), deserialized.id());
        assert_eq!(a.slug(), deserialized.slug());
        assert_eq!(a.summary(), deserialized.summary());
        assert_eq!(a.title(), deserialized.title());
        assert_eq!(a.status(), deserialized.status());
        assert_eq!(a.content().len(), deserialized.content().len());
        assert_eq!(a.thumbnail().url(), deserialized.thumbnail().url());
        assert_eq!(a.thumbnail().alt(), deserialized.thumbnail().alt());
        assert_eq!(a.thumbnail().title(), deserialized.thumbnail().title());
        assert_eq!(a.thumbnail().width(), deserialized.thumbnail().width());
        assert_eq!(a.thumbnail().height(), deserialized.thumbnail().height());
        assert_eq!(a.category().title(), deserialized.category().title());
        assert_eq!(a.category().emoji(), deserialized.category().emoji());
        assert_eq!(a.metatags().title(), deserialized.metatags().title());
        assert_eq!(
            a.metatags().description(),
            deserialized.metatags().description()
        );
        assert_eq!(a.metatags().keywords(), deserialized.metatags().keywords());
    }

    pub fn article_fixture() -> Article {
        ArticleBuilder::default()
            .id("e1f2a3b4-c5d6-4e7f-8a9b-0c1d2e3f4a5b".try_into().unwrap())
            .slug("/what-is-lorem-ipsum".try_into().unwrap())
            .title("What is Lorem Ipsum?".try_into().unwrap())
            .summary("Lorem Ipsum is simply dummy text".try_into().unwrap())
            .status(ModerationStatus::Published)
            .created_at("2024-12-15T14:03:56+00:00".try_into().unwrap())
            .category(category_fixture())
            .thumbnail(image_fixture())
            .metatags(metatags_fixture())
            .content(vec![])
            .build()
            .unwrap()
    }

    pub fn unpublished_article_fixture() -> Article {
        ArticleBuilder::default()
            .id("e1f2a3b4-c5d6-4e7f-8a9b-0c1d2e3f4a5b".try_into().unwrap())
            .slug("/what-is-lorem-ipsum".try_into().unwrap())
            .title("What is Lorem Ipsum Draft?".try_into().unwrap())
            .summary("Lorem Ipsum is simply dummy text".try_into().unwrap())
            .status(ModerationStatus::Unpublished)
            .created_at("2024-12-15T14:03:56+00:00".try_into().unwrap())
            .category(category_fixture())
            .thumbnail(image_fixture())
            .metatags(metatags_fixture())
            .content(vec![])
            .build()
            .unwrap()
    }

    pub fn invalid_article_fixture() -> Article {
        ArticleBuilder::default()
            .title("".try_into().unwrap())
            .summary("".try_into().unwrap())
            .created_at("".try_into().unwrap())
            .status(false.into())
            .category(category_fixture())
            .thumbnail(image_fixture())
            .metatags(metatags_fixture())
            .content(vec![])
            .build()
            .unwrap()
    }

    pub fn category_fixture() -> Category {
        CategoryBuilder::default()
            .id(Identifier::try_from("f5e4d3c2-b1a0-4f9e-8d7c-6b5a4c3d2e1f").unwrap())
            .slug(Url::try_from("/example").unwrap())
            .title(RequiredText::try_from("Example").unwrap())
            .status(ModerationStatus::Published)
            .emoji(RequiredText::try_from("⌨️").unwrap())
            .build()
            .unwrap()
    }
}
