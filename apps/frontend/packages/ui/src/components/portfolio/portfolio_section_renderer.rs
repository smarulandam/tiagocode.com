use dioxus::prelude::*;

use super::{AboutMeSection, FeaturedArticlesSection, ProjectsSection, ResumeSection};
use crate::components::common::UnsupportedSection;
use content_core::application::domain::portfolio::PortfolioSection;

#[component]
pub fn PortfolioSectionRenderer(sections: Vec<PortfolioSection>) -> Element {
    rsx! {
        for section in sections {
            match section {
                PortfolioSection::AboutMe(section) => rsx! {
                    AboutMeSection {
                        subtitle: section.subtitle().to_string(),
                        title: section.title().to_string(),
                        text: section.text().to_string(),
                        skills: section.skills().clone(),
                        profile_picture: section.profile_picture().clone(),
                        years_of_experience: *section.years_of_experience(),
                        cv_document: section.cv_document().clone(),
                    }
                },
                PortfolioSection::Resume(section) => rsx! {
                    ResumeSection {
                        title: section.title().to_string(),
                        subtitle: section.subtitle().to_string(),
                        text: section.text().to_string(),
                        education: section.education().clone(),
                        experience: section.experience().clone(),
                    }
                },
                PortfolioSection::Projects(section) => rsx! {
                    ProjectsSection {
                        title: section.title().to_string(),
                        subtitle: section.subtitle().to_string(),
                        text: section.text().to_string(),
                        projects: section.projects().clone(),
                    }
                },
                PortfolioSection::Blogs(section) => rsx! {
                    FeaturedArticlesSection {
                        title: section.title().to_string(),
                        subtitle: section.subtitle().to_string(),
                        text: section.text().to_string(),
                        articles: section.articles().clone(),
                    }
                },
                PortfolioSection::Unknown => rsx! {
                    UnsupportedSection {}
                },
            }
        }
    }
}
