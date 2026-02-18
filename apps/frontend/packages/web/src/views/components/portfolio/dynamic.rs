use dioxus::prelude::*;

use content_core::application::domain::portfolio::PortfolioSection;

use crate::views::components::common::MissingSection;
use crate::views::components::portfolio::{
    AboutMeSection, BlogsSection, ProjectsSection, ResumeSection,
};

#[component]
pub fn DynamicSections(sections: Vec<PortfolioSection>, lang: String) -> Element {
    rsx! {
        for section in sections {
            match section {
                PortfolioSection::AboutMe(data) => rsx! { AboutMeSection { data } },
                PortfolioSection::Resume(data) => rsx! { ResumeSection { data } },
                PortfolioSection::Projects(data) => rsx! { ProjectsSection { data } },
                PortfolioSection::Blogs(data) => rsx! { BlogsSection { data, lang: lang.clone() } },
                PortfolioSection::Unknown => rsx! { MissingSection {} },
            }
        }
    }
}
