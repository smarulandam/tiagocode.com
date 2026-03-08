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
            <div class="flex flex-col gap-8 lg:flex-row lg:gap-10">
                <div class="lg:w-[18rem]">
                    <div class="profile-highlight">
                        <div class="profile-avatar">
                            <Img image=profile_picture class="h-full w-full object-cover" />
                        </div>
                        <div class="experience-badge">
                            <div class="experience-badge__number">
                                <span class="counter">{years_of_experience}</span>
                            </div>
                            <div class="experience-badge__label">Years of Experience</div>
                        </div>
                    </div>
                </div>
                <div class="flex-1">
                    <div class="section-heading">
                        <Decoration text=subtitle />
                        <PrimaryTitle text=title />
                    </div>
                    <Description text=text />
                    <div class="mt-6 flex flex-wrap gap-3">
                        {skills.into_iter().map(|skill| view! { <Pill text=skill.to_string() /> }).collect_view()}
                    </div>
                    <div class="mt-8 flex justify-start">
                        <a
                            href=cv_document.url().to_string()
                            class="button-secondary"
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
