use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::value_objects::{Date, Url};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Sitemap {
    entries: Vec<SitemapEntry>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct SitemapEntry {
    location: Url,
    last_modification: Option<Date>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::application::domain::sitemap::{SitemapBuilder, SitemapEntryBuilder};

    #[test]
    fn serialization_succeeds_when_valid_sitemap() {
        let sitemap = sitemap_fixture();
        let serialized = serde_json::to_string(&sitemap).unwrap();

        assert!(serialized.contains(sitemap.entries()[0].location().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_sitemap_entry() {
        let e = sitemap_entry_fixture();
        let serialized = serde_json::to_string(&e).unwrap();

        assert!(serialized.contains(e.location().as_str()));
    }

    pub fn sitemap_fixture() -> Sitemap {
        SitemapBuilder::default()
            .entries(vec![sitemap_entry_fixture()])
            .build()
            .unwrap()
    }

    pub fn sitemap_entry_fixture() -> SitemapEntry {
        SitemapEntryBuilder::default()
            .location("/en/articles/rust/what-is-ownership".try_into().unwrap())
            .last_modification(Some("2026-04-02T08:15:00+00:00".try_into().unwrap()))
            .build()
            .unwrap()
    }
}
