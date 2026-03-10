use dioxus::prelude::*;

use content_core::application::domain::common::MetaTags as Props;

#[component]
pub fn MetaTags(metatags: Props) -> Element {
    rsx! {
        Title { "{metatags.title()}" }

        Meta {
            name: "description",
            content: metatags.description().to_string()
        }
        Meta {
            name: "keywords",
            content: metatags.keywords().to_string()
        }
        Meta {
            name: "robots",
            content: metatags.robots().to_string()
        }
        Meta {
            property: "og:type",
            content: metatags.og_type().to_string()
        }
        Meta {
            property: "og:site_name",
            content: metatags.og_site_name().to_string(),
        }
        Meta {
            property: "og:title",
            content: metatags.og_title().to_string()
        }
        Meta {
            property: "og:description",
            content: metatags.og_description().to_string(),
        }
        Meta {
            property: "og:image",
            content: metatags.og_image().to_string()
        }
        Meta {
            property: "og:url",
            content: metatags.og_url().to_string()
        }
        Meta {
            name: "twitter:card",
            content: metatags.twitter_card().to_string(),
        }
        Meta {
            name: "twitter:title",
            content: metatags.twitter_title().to_string(),
        }
        Meta {
            name: "twitter:description",
            content: metatags.twitter_description().to_string(),
        }
        Meta {
            name: "twitter:image",
            content: metatags.twitter_image().to_string(),
        }
        Meta {
            name: "twitter:site",
            content: metatags.twitter_site().to_string(),
        }

        document::Link { rel: "canonical", href: metatags.canonical_url().to_string() }
    }
}
