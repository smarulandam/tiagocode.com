use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::common::{Image, Link, Project};

#[component]
pub fn ProjectsSection(
    title: String,
    subtitle: String,
    text: String,
    projects: Vec<Project>,
) -> impl IntoView {
    view! {
        <Container id="portfolio".into()>
            <div class="section-heading">
                <Decoration text=subtitle />
                <SecondaryTitle text=title />
                <Description text=text />
            </div>
            <div class="mt-8">
                <div class="project-grid">
                    {projects
                    .into_iter()
                    .map(|p| {
                        view! {
                            <FeaturedProjectCard
                                title=p.title().to_string()
                                link=p.link().clone()
                                image=p.image().clone()
                            />
                        }
                    }).collect_view()}
                </div>
            </div>
        </Container>
    }
}

#[component]
pub fn FeaturedProjectCard(title: String, link: Link, image: Image) -> impl IntoView {
    view! {
        <article class="project-card">
            <a target="_blank" href=link.url().to_string() rel="noopener noreferrer">
                <Img image=image class="h-full w-full object-cover" />
                <div class="project-card__title">{title.to_string()}</div>
            </a>
        </article>
    }
}
