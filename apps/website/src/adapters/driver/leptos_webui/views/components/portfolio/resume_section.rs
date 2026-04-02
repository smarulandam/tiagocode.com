use leptos::prelude::*;

use super::{TimelineEntry, TimelineGroup};
use crate::adapters::driver::leptos_webui::views::components::common::{
    SectionContainer, SectionDescription, SectionEyebrow, SectionTitle,
};
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
        <SectionContainer id="resume".into()>
            <div class="">
                <SectionEyebrow text=subtitle />
                <SectionTitle text=title />
                <SectionDescription text=text />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-8 lg:gap-10 mt-6 lg:mt-12">
                <TimelineGroup icon="mortarboard">
                    {education
                        .items()
                        .into_iter()
                        .map(|education_item| {
                            view! {
                                <TimelineEntry
                                    date=education_item.date().clone().to_string()
                                    title=education_item.title().to_string()
                                    subtitle=education_item.subtitle().to_string()
                                />
                            }
                        })
                        .collect_view()}
                </TimelineGroup>
                <TimelineGroup icon="briefcase">
                    {experience
                        .items()
                        .into_iter()
                        .map(|experience_item| {
                            view! {
                                <TimelineEntry
                                    date=experience_item.date().to_string()
                                    title=experience_item.title().to_string()
                                    subtitle=experience_item.subtitle().to_string()
                                />
                            }
                        })
                        .collect_view()}
                </TimelineGroup>
            </div>
        </SectionContainer>
    }
}
