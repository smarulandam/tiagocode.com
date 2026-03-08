use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::common::{Document, Image};

#[component]
pub fn AboutMeSection(
    subtitle: String,
    title: String,
    text: String,
    skills: Vec<String>,
    profile_picture: Image,
    years_of_experience: u8,
    cv_document: Document,
) -> impl IntoView {
    view! {
        <Container id="about".into()>
            <div class="flex flex-col gap-5 lg:flex-row lg:gap-10">
                <div>
                    <div class="flex justify-center relative h-fit">
                        <Img image=profile_picture class="min-w-52 min-h-52 max-w-64 max-h-64 rounded-full" />
                    </div>
                    <div class="flex items-center justify-center">
                        <div class="pe-2">
                            <div class="font-mono font-semibold text-6xl stroke-text">
                                <span class="counter">{years_of_experience}</span>
                            </div>
                        </div>
                        <div class="py-2">
                            <span class="block text-2xl font-normal mb-1">+</span>
                            <p class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Years of Experience</p>
                        </div>
                    </div>
                </div>
                <div>
                    <Decoration text=subtitle />
                    <PrimaryTitle text=title />
                    <Description text=text />
                    <div class="mb-2 flex flex-wrap">
                        {skills.into_iter().map(|skill| view! { <Pill text=skill.to_string() /> }).collect_view()}
                    </div>
                    <div class="mb-2 flex justify-end">
                        <a
                            href=cv_document.url().to_string()
                            class="inline-flex items-center rounded-full border border-black border-dashed px-6 py-3 font-mono text-sm text-zeus transition ease-out duration-[120ms] hover:bg-black hover:text-white"
                            target="_blank"
                        >
                            <span>"Download cv"</span>
                            <i class="bi bi-cloud-download ps-1"></i>
                        </a>
                    </div>
                </div>
            </div>
        </Container>
    }
}
