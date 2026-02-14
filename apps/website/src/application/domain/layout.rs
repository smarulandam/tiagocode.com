use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::common::Image;
use crate::application::value_objects::{Identifier, Url};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Layout {
    logo: Option<Image>,
    main_menu: MenuTree,
    social_menu: MenuTree,
    sidebar_menu: Option<MenuTree>,
    footer_menu: Option<MenuTree>,
}

#[derive(Serialize, Deserialize, Builder, Getters, Clone, Debug)]
pub struct MenuTree {
    items: Vec<MenuItem>,
}

#[derive(Serialize, Deserialize, Builder, Getters, Clone, Debug)]
pub struct MenuItem {
    id: Identifier,
    url: Url,
    title: String,
    hidden: bool,
    weight: isize,
    icon: Option<Image>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::application::domain::common::tests::image_fixture;

    #[test]
    fn creation_succeeds_when_valid_menu_item() {
        menu_item_fixture(); // the fixture calls the builder methods, so we just ensure that it doesn't panic
    }

    #[test]
    fn serialization_succeeds_when_valid_menu_item() {
        let m = menu_item_fixture();
        let serialized = serde_json::to_string(&m).unwrap();

        assert!(serialized.contains(m.id().to_string().as_str()));
        assert!(serialized.contains(m.url().as_str()));
        assert!(serialized.contains(m.title().as_str()));
        assert!(serialized.contains(m.hidden().to_string().as_str()));
        assert!(serialized.contains(m.weight().to_string().as_str()));
        assert!(serialized.contains(m.icon().clone().unwrap().id().to_string().as_str()));
        assert!(serialized.contains(m.icon().clone().unwrap().url().as_str()));
        assert!(serialized.contains(m.icon().clone().unwrap().title().as_str()));
        assert!(serialized.contains(m.icon().clone().unwrap().alt().as_str()));
        assert!(serialized.contains(m.icon().clone().unwrap().width().to_string().as_str()));
        assert!(serialized.contains(m.icon().clone().unwrap().height().to_string().as_str()));
    }

    #[test]
    fn deserialization_succeeds_for_menu_item() {
        let m = menu_item_fixture();
        let serialized = serde_json::to_string(&m).unwrap();
        let deserialized: MenuItem = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id(), m.id());
        assert_eq!(deserialized.url(), m.url());
        assert_eq!(deserialized.title(), m.title());
        assert_eq!(deserialized.hidden(), m.hidden());
        assert_eq!(deserialized.weight(), m.weight());
        assert!(deserialized.icon().is_some());
    }

    pub fn menu_item_fixture() -> MenuItem {
        MenuItemBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .url("https://example.com".try_into().unwrap())
            .title("Example".into())
            .hidden(false)
            .weight(0)
            .icon(Some(image_fixture()))
            .build()
            .unwrap()
    }
}
