use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::value_objects::{Identifier, RequiredText, Url};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Project {
    id: Identifier,
    title: RequiredText,
    text: String,
    link: Link,
    image: Image,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Timeline {
    id: Identifier,
    items: Vec<TimelineItem>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct TimelineItem {
    id: Identifier,
    date: RequiredText,
    title: RequiredText,
    subtitle: RequiredText,
    text: Option<String>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Image {
    id: Identifier,
    url: Url,
    alt: RequiredText,
    title: RequiredText,
    width: u16,
    height: u16,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Document {
    id: Identifier,
    url: Url,
    mime: RequiredText,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Link {
    url: Url,
    title: String,
    options: Vec<(String, String)>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct MetaTags {
    title: RequiredText,
    description: RequiredText,
    keywords: RequiredText,
    canonical_url: Url,
    robots: RequiredText,
    og_type: RequiredText,
    og_site_name: RequiredText,
    og_title: RequiredText,
    og_description: RequiredText,
    og_image: Url,
    og_url: Url,
    twitter_card: RequiredText,
    twitter_title: RequiredText,
    twitter_description: RequiredText,
    twitter_image: Url,
    twitter_site: RequiredText,
    twitter_creator: RequiredText,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn creation_succeeds_when_valid_timeline() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        timeline_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_project() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        project_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_image() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        image_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_document() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        document_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_link() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        link_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_metatags() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        metatags_fixture();
    }

    #[test]
    fn serialization_succeeds_when_valid_timeline() {
        let t = timeline_fixture();
        let first_item = t.items().first().unwrap();
        let serialized = serde_json::json!(&t).to_string();

        assert!(serialized.contains(t.id().to_string().as_str()));
        assert!(serialized.contains(first_item.id().to_string().as_str()));
        assert!(serialized.contains(first_item.title().to_string().as_str()));
        assert!(serialized.contains(first_item.subtitle().to_string().as_str()));
        assert!(serialized.contains(first_item.text().clone().unwrap().to_string().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_project() {
        let p = project_fixture();
        let serialized = serde_json::json!(&p).to_string();

        assert!(serialized.contains(p.id().to_string().as_str()));
        assert!(serialized.contains(p.title().to_string().as_str()));
        assert!(serialized.contains(p.text().as_str()));
        assert!(serialized.contains(p.link().url().as_str()));
        assert!(serialized.contains(p.image().url().as_str()));
        assert!(serialized.contains(p.image().alt().to_string().as_str()));
        assert!(serialized.contains(p.image().title().to_string().as_str()));
        assert!(serialized.contains(p.image().width().to_string().as_str()));
        assert!(serialized.contains(p.image().height().to_string().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_image() {
        let i = image_fixture();
        let serialized = serde_json::json!(&i).to_string();

        assert!(serialized.contains(i.id().to_string().as_str()));
        assert!(serialized.contains(i.url().as_str()));
        assert!(serialized.contains(i.alt().to_string().as_str()));
        assert!(serialized.contains(i.title().to_string().as_str()));
        assert!(serialized.contains(i.width().to_string().as_str()));
        assert!(serialized.contains(i.height().to_string().as_str()));
    }

    #[test]
    fn serialization_succeeds_for_document() {
        let d = document_fixture();
        let serialized = serde_json::json!(&d).to_string();

        assert!(serialized.contains(d.id().to_string().as_str()));
        assert!(serialized.contains(d.url().as_str()));
        assert!(serialized.contains(d.mime().to_string().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_link() {
        let l = link_fixture();
        let serialized = serde_json::json!(&l).to_string();

        assert!(serialized.contains(l.url().as_str()));
        assert!(serialized.contains(l.title().as_str()));
        assert!(serialized.contains("[]"));
    }

    #[test]
    fn serialization_succeeds_when_valid_metatags() {
        let m = metatags_fixture();
        let serialized = serde_json::json!(&m).to_string();

        assert!(serialized.contains(m.title().as_str()));
        assert!(serialized.contains(m.description().as_str()));
        assert!(serialized.contains(m.keywords().as_str()));
        assert!(serialized.contains(m.canonical_url().as_str()));
        assert!(serialized.contains(m.robots().as_str()));
        assert!(serialized.contains(m.og_type().as_str()));
        assert!(serialized.contains(m.og_site_name().as_str()));
        assert!(serialized.contains(m.og_title().as_str()));
        assert!(serialized.contains(m.og_description().as_str()));
        assert!(serialized.contains(m.og_image().as_str()));
        assert!(serialized.contains(m.og_url().as_str()));
        assert!(serialized.contains(m.twitter_card().as_str()));
        assert!(serialized.contains(m.twitter_title().as_str()));
        assert!(serialized.contains(m.twitter_description().as_str()));
        assert!(serialized.contains(m.twitter_image().as_str()));
        assert!(serialized.contains(m.twitter_site().as_str()));
        assert!(serialized.contains(m.twitter_creator().as_str()));
    }

    fn link_fixture() -> Link {
        LinkBuilder::default()
            .url("https://example.com".try_into().unwrap())
            .title("Example link".to_string())
            .options(vec![])
            .build()
            .unwrap()
    }

    pub fn document_fixture() -> Document {
        DocumentBuilder::default()
            .id("a1b2c3d4-5e6f-7a8b-9c0d-1e2f3a4b5c6d".try_into().unwrap())
            .url("https://example.com/example.pdf".try_into().unwrap())
            .mime("application/pdf".try_into().unwrap())
            .build()
            .unwrap()
    }

    pub fn image_fixture() -> Image {
        ImageBuilder::default()
            .id("b2c3d4e5-6f7a-8b9c-0d1e-2f3a4b5c6d7e".try_into().unwrap())
            .url("https://example.com/example.png".try_into().unwrap())
            .alt("Dummy image".try_into().unwrap())
            .title("Dummy image".try_into().unwrap())
            .height(500)
            .width(500)
            .build()
            .unwrap()
    }

    pub fn project_fixture() -> Project {
        ProjectBuilder::default()
            .id("c3d4e5f6-7a8b-9c0d-1e2f-3a4b5c6d7e8f".try_into().unwrap())
            .title("Example project".try_into().unwrap())
            .text("Example project description".to_string())
            .link(link_fixture())
            .image(image_fixture())
            .build()
            .unwrap()
    }

    pub fn timeline_fixture() -> Timeline {
        TimelineBuilder::default()
            .id("d4e5f6a7-8b9c-0d1e-2f3a-4b5c6d7e8f9a".try_into().unwrap())
            .items(vec![timeline_item_fixture(), timeline_item_fixture()])
            .build()
            .unwrap()
    }

    fn timeline_item_fixture() -> TimelineItem {
        TimelineItemBuilder::default()
            .id("e5f6a7b8-9c0d-1e2f-3a4b-5c6d7e8f9a0b".try_into().unwrap())
            .date("2024-12-15 14:03:56".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text(Some("Example text".to_string()))
            .build()
            .unwrap()
    }

    pub fn metatags_fixture() -> MetaTags {
        MetaTagsBuilder::default()
            .title("Example Title".try_into().unwrap())
            .description("Example description".try_into().unwrap())
            .keywords("example, rust, code".try_into().unwrap())
            .canonical_url("https://example.com".try_into().unwrap())
            .robots("index, follow".try_into().unwrap())
            .og_type("website".try_into().unwrap())
            .og_site_name("Example Site".try_into().unwrap())
            .og_title("Example Open Graph Title".try_into().unwrap())
            .og_description("Example Open Graph Description".try_into().unwrap())
            .og_image("https://example.com/image.png".try_into().unwrap())
            .og_url("https://example.com/page".try_into().unwrap())
            .twitter_card("summary_large_image".try_into().unwrap())
            .twitter_title("Example Twitter Title".try_into().unwrap())
            .twitter_description("Example Twitter Description".try_into().unwrap())
            .twitter_image("https://example.com/twitter-image.png".try_into().unwrap())
            .twitter_site("@example".try_into().unwrap())
            .twitter_creator("@creator".try_into().unwrap())
            .build()
            .unwrap()
    }
}
