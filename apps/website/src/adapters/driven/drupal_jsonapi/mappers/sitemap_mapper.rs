use crate::adapters::driven::drupal_jsonapi::entities::SitemapNode;
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::sitemap::{
    Sitemap, SitemapBuilder, SitemapEntry, SitemapEntryBuilder,
};

pub trait ExternalSitemapAdapter {
    type Input;

    fn adapt(&self, input: Self::Input) -> Result<SitemapEntry>;

    fn adapt_multiple(&self, input: Vec<Self::Input>) -> Result<Sitemap> {
        SitemapBuilder::default()
            .entries(
                input
                    .into_iter()
                    .filter_map(|entry| self.adapt(entry).ok())
                    .collect(),
            )
            .build()
            .map_err(AppError::unexpected)
    }
}

#[derive(Default)]
pub struct SitemapEntryMapper;

impl ExternalSitemapAdapter for SitemapEntryMapper {
    type Input = SitemapNode;

    fn adapt(&self, input: Self::Input) -> Result<SitemapEntry> {
        sitemap_entry_mapper(input)
    }
}

fn sitemap_entry_mapper(entry: SitemapNode) -> Result<SitemapEntry> {
    let mut builder = SitemapEntryBuilder::default();
    builder.location(entry.attributes().path().as_str().try_into()?);
    builder.last_modification(None);

    if let Some(lastmod) = entry
        .attributes()
        .lastmod()
        .clone()
        .filter(|value| !value.trim().is_empty())
    {
        builder.last_modification(Some(lastmod.try_into()?));
    }

    builder.build().map_err(AppError::unexpected)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sitemap_resource_fixture(path: &str, lastmod: Option<&str>) -> SitemapNode {
        serde_json::from_value(serde_json::json!({
            "id": "entry-1",
            "type": "sitemap--entry",
            "attributes": {
                "path": path,
                "lastmod": lastmod,
            }
        }))
        .unwrap()
    }

    #[test]
    fn adapter_succeeds_when_input_is_valid() {
        let entry = SitemapEntryMapper
            .adapt(sitemap_resource_fixture(
                "/en/articles/rust/what-is-ownership",
                Some("2026-04-02T08:15:00+00:00"),
            ))
            .unwrap();

        assert_eq!(
            entry.location().as_str(),
            "/en/articles/rust/what-is-ownership"
        );
        assert_eq!(
            entry.last_modification().as_ref().unwrap().to_rfc3339(),
            "2026-04-02T08:15:00+00:00"
        );
    }

    #[test]
    fn adapter_multiple_skips_invalid_entries() {
        let entries = SitemapEntryMapper
            .adapt_multiple(vec![
                sitemap_resource_fixture("/en", Some("2026-04-02T08:15:00+00:00")),
                sitemap_resource_fixture("invalid-path", Some("2026-04-02T08:15:00+00:00")),
                sitemap_resource_fixture(
                    "/es/articles/inteligencia-artificial",
                    Some("invalid-date"),
                ),
            ])
            .unwrap();

        assert_eq!(entries.entries().len(), 1);
        assert_eq!(entries.entries()[0].location().as_str(), "/en");
    }

    #[test]
    fn adapter_ignores_empty_lastmod() {
        let entry = SitemapEntryMapper
            .adapt(sitemap_resource_fixture("/en", Some("")))
            .unwrap();

        assert!(entry.last_modification().is_none());
    }
}
