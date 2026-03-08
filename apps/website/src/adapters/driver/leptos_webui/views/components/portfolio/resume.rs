use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::adapters::driver::leptos_webui::views::components::portfolio::{Timeline, TimelineItem};
use crate::application::domain::common::Timeline;

#[component]
pub fn ResumeSection(
    title: String,
    subtitle: String,
    text: String,
    education: Timeline,
    experience: Timeline,
) -> impl IntoView {
    view! {
        <Container id="resume".into()>
            <div class="">
                <Decoration text=subtitle />
                <SecondaryTitle text=title />
                <Description text=text />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-8 lg:gap-10 mt-6 lg:mt-12">
                <Timeline icon="bi bi-mortarboard">
                    {education
                    .items()
                    .into_iter()
                    .map(|e| {
                        view! {
                            <TimelineItem
                                date=e.date().clone().to_string()
                                title=e.title().to_string()
                                subtitle=e.subtitle().to_string()
                            />
                        }
                    }).collect_view()}
                </Timeline>
                <Timeline icon="bi bi-briefcase">
                    {experience
                    .items()
                    .into_iter()
                    .map(|e| {
                        view! {
                            <TimelineItem
                                date=e.date().to_string()
                                title=e.title().to_string()
                                subtitle=e.subtitle().to_string()
                            />
                        }
                    }).collect_view()}
                </Timeline>
            </div>
        </Container>
    }
}
