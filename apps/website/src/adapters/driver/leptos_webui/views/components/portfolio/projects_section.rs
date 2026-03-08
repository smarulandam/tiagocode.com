use leptos::prelude::*;

use super::FeaturedProjectCard;
use crate::adapters::driver::leptos_webui::views::components::common::{
    SectionContainer, SectionDescription, SectionEyebrow, SectionTitle,
};
use crate::application::domain::common::Project;

#[component]
pub fn ProjectsSection(
    title: String,
    subtitle: String,
    text: String,
    projects: Vec<Project>,
) -> impl IntoView {
    view! {
        <SectionContainer id="portfolio".into()>
            <div class="">
                <SectionEyebrow text=subtitle />
                <SectionTitle text=title />
                <SectionDescription text=text />
            </div>
            <div class="mt-6 lg:mt-12">
                <div class="portfolio-grid grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
                    {projects
                        .into_iter()
                        .map(|project| {
                            view! {
                                <FeaturedProjectCard
                                    title=project.title().to_string()
                                    link=project.link().clone()
                                    image=project.image().clone()
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </SectionContainer>
    }
}
