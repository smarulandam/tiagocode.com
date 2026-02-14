use crate::adapters::driven::drupal_jsonapi::entities::{MetatagAttributesField, MetatagsField};
use crate::application::domain::common::{MetaTags, MetaTagsBuilder};

pub fn metatags_field_mapper(metatags: &Vec<MetatagsField>) -> MetaTags {
    MetaTagsBuilder::default()
        .title(
            extract_metatag_content(metatags, "title")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .description(
            extract_metatag_content(metatags, "description")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .keywords(
            extract_metatag_content(metatags, "keywords")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .canonical_url(
            extract_metatag_content(metatags, "canonical")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .robots(
            extract_metatag_content(metatags, "robots")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .og_type(
            extract_metatag_content(metatags, "og:type")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .og_site_name(
            extract_metatag_content(metatags, "og:site_name")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .og_title(
            extract_metatag_content(metatags, "og:title")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .og_description(
            extract_metatag_content(metatags, "og:description")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .og_url(
            extract_metatag_content(metatags, "shortlink")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .og_image(
            extract_metatag_content(metatags, "og:image:secure_url")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .twitter_card(
            extract_metatag_content(metatags, "twitter:card")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .twitter_title(
            extract_metatag_content(metatags, "twitter:title")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .twitter_creator(
            extract_metatag_content(metatags, "twitter:site")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .twitter_description(
            extract_metatag_content(metatags, "twitter:description")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .twitter_image(
            extract_metatag_content(metatags, "og:image:secure_url")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .twitter_site(
            extract_metatag_content(metatags, "twitter:site")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .build()
        .unwrap()
}

fn extract_metatag_content<'a>(metatags: &'a Vec<MetatagsField>, key: &str) -> Option<&'a str> {
    metatags
        .iter()
        .find_map(|metatag| match &metatag.attributes() {
            MetatagAttributesField::Named { name, content } if name == key => {
                Some(content.as_str())
            }
            MetatagAttributesField::Property { property, content } if property == key => {
                Some(content.as_str())
            }
            MetatagAttributesField::Link { rel, href } if rel == key => Some(href.as_str()),
            _ => None,
        })
}
