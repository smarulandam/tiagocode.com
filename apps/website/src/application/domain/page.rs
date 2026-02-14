use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::common::MetaTags;
use crate::application::value_objects::{Date, Identifier, ModerationStatus, RequiredText};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Builder)]
pub struct Page {
    id: Identifier,
    title: RequiredText,
    created_at: Date,
    status: ModerationStatus,
    metatags: MetaTags,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::application::domain::common::tests::metatags_fixture;

    #[test]
    fn creation_succeeds_when_valid_page() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        page_fixture();
    }

    #[test]
    fn serialization_succeeds_when_valid_page() {
        let p = page_fixture();
        let serialized = serde_json::json!(&p).to_string();

        assert!(serialized.contains(p.id().to_string().as_str()));
        assert!(serialized.contains(p.title().to_string().as_str()));
        assert!(serialized.contains(p.status().to_string().as_str()));
        assert!(serialized.contains("metatags"));
    }

    pub fn page_fixture() -> Page {
        PageBuilder::default()
            .id(Identifier::try_from("413b8ba1-2bc4-4fda-8455-0c0dea528ca0").unwrap())
            .title(RequiredText::try_from("Test Page").unwrap())
            .metatags(metatags_fixture())
            .status(ModerationStatus::Published)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .build()
            .unwrap()
    }
}
