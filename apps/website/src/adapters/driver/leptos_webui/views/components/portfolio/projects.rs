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
            <div class="">
                <Decoration text=subtitle />
                <SecondaryTitle text=title />
                <Description text=text />
            </div>
            <div class="mt-6 lg:mt-12">
                <div class="portfolio-grid grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
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
        <div class="portfolio-item category-1">
            <div class="relative overflow-hidden group rounded-lg after:content-[''] after:absolute after:top-0 after:left-0 after:w-full after:h-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:ease-out after:duration-[160ms] hover:after:opacity-100 category-1">
               <Img image=image class="transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.4px]" />
                <div class="z-[1] text-center absolute bottom-0 left-0 w-full px-2 pb-6 invisible opacity-0 translate-y-2 group-hover:translate-y-0 group-hover:visible group-hover:opacity-100 group-hover:mb-0 transition ease-out duration-[160ms]">
                    <a target="_blank" href=link.url().to_string() class="font-poppins font-semibold text-3xl lg:text-4xl tracking-[0.5px] text-white transition-all ease-linear duration-100">
                        {title.to_string()}
                    </a>
                </div>
            </div>
        </div>
    }
}
