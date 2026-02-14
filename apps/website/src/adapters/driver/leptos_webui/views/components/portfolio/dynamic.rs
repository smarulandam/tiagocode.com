use leptos::either::EitherOf5;
use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::MissingSection;
use crate::adapters::driver::leptos_webui::views::components::portfolio::*;
use crate::application::domain::portfolio::PortfolioSection;

#[component]
pub fn DynamicSections(sections: Vec<PortfolioSection>) -> impl IntoView {
    view! {
        {sections
            .into_iter()
            .map(|content| {
                match content {
                    PortfolioSection::AboutMe(section) => {
                        EitherOf5::A(
                            view! {
                                <AboutMeSection
                                    subtitle=section.subtitle().to_string()
                                    title=section.title().to_string()
                                    text=section.text().to_string()
                                    skills=section.skills().clone()
                                    profile_picture=section.profile_picture().clone()
                                    years_of_experience=section.years_of_experience().clone()
                                    cv_document=section.cv_document().clone()
                                />
                            },
                        )
                    }
                    PortfolioSection::Resume(section) => {
                        EitherOf5::B(
                            view! {
                                <ResumeSection
                                    title=section.title().to_string()
                                    subtitle=section.subtitle().to_string()
                                    text=section.text().to_string()
                                    education=section.education().clone()
                                    experience=section.experience().clone()
                                />
                            },
                        )
                    }
                    PortfolioSection::Projects(section) => {
                        EitherOf5::C(
                            view! {
                                <ProjectsSection
                                    title=section.title().to_string()
                                    subtitle=section.subtitle().to_string()
                                    text=section.text().to_string()
                                    projects=section.projects().clone()
                                />
                            },
                        )
                    },
                    PortfolioSection::Blogs(section) => {
                        EitherOf5::D(
                            view! {
                                <BlogSection
                                    title=section.title().to_string()
                                    subtitle=section.subtitle().to_string()
                                    text=section.text().to_string()
                                    articles=section.articles().clone()
                                />
                            },
                        )
                    },
                    _ => EitherOf5::E(view! { <MissingSection /> }),
                }
            }).collect_view()
        }
    }
}
