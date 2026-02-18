use dioxus::prelude::*;

use content_core::application::domain::common::MetaTags;

#[component]
pub fn MetaTagsView(metatags: MetaTags) -> Element {
    rsx! {
        document::Title { "{metatags.title()}" }

        document::Meta { name: "description", content: metatags.description().to_string() }
        document::Meta { name: "keywords", content: metatags.keywords().to_string() }
        document::Meta { name: "robots", content: metatags.robots().to_string() }

        document::Meta { property: "og:type", content: metatags.og_type().to_string() }
        document::Meta { property: "og:site_name", content: metatags.og_site_name().to_string() }
        document::Meta { property: "og:title", content: metatags.og_title().to_string() }
        document::Meta { property: "og:description", content: metatags.og_description().to_string() }
        document::Meta { property: "og:image", content: metatags.og_image().to_string() }
        document::Meta { property: "og:url", content: metatags.og_url().to_string() }

        document::Meta { name: "twitter:card", content: metatags.twitter_card().to_string() }
        document::Meta { name: "twitter:title", content: metatags.twitter_title().to_string() }
        document::Meta { name: "twitter:description", content: metatags.twitter_description().to_string() }
        document::Meta { name: "twitter:image", content: metatags.twitter_image().to_string() }
        document::Meta { name: "twitter:site", content: metatags.twitter_site().to_string() }

        document::Link { rel: "canonical", href: metatags.canonical_url().to_string() }
    }
}
