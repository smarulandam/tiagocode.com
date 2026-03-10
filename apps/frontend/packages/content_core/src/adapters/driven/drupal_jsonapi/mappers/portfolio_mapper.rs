use voca_rs::strip::strip_tags;

use crate::adapters::driven::drupal_jsonapi::entities::ContentTimelineItemParagraph;
use crate::adapters::driven::drupal_jsonapi::entities::{ContentField, DocumentField};
use crate::adapters::driven::drupal_jsonapi::entities::{ImageField, PortfolioNode};
use crate::adapters::driven::drupal_jsonapi::mappers::metatags_field_mapper;
use crate::application::domain::common::{Document, DocumentBuilder, Image, ImageBuilder};
use crate::application::domain::common::{LinkBuilder, Project, ProjectBuilder};
use crate::application::domain::common::{Timeline, TimelineBuilder};
use crate::application::domain::common::{TimelineItem, TimelineItemBuilder};
use crate::application::domain::core::{AppError, Result};
use crate::application::domain::portfolio::{AboutMeBuilder, BlogsBuilder};
use crate::application::domain::portfolio::{Portfolio, PortfolioBuilder, PortfolioSection};
use crate::application::domain::portfolio::{ProjectsBuilder, ResumeBuilder};

/// Trait for converting external data into a `Portfolio` domain entity.
/// Ensures separation between external data sources and core domain logic.
///
/// # Associated Types
/// - `Input`: The external data type to be transformed into a `Portfolio`.
pub trait ExternalPortfolioAdapter {
    type Input;

    /// Converts external data into a `Portfolio`.
    ///
    /// # Arguments
    /// * `input` - The external data to be transformed into a `Portfolio`.
    ///
    /// # Returns
    /// * `ApplicationResult<Portfolio>` - The result of the transformation.
    fn adapt(&self, input: Self::Input) -> Result<Portfolio>;
}

#[derive(Default)]
pub struct PortfolioNodeMapper;

impl ExternalPortfolioAdapter for PortfolioNodeMapper {
    type Input = PortfolioNode;

    fn adapt(&self, input: Self::Input) -> Result<Portfolio> {
        Ok(portfolio_node_mapper(input)?)
    }
}

fn portfolio_node_mapper(node: PortfolioNode) -> Result<Portfolio> {
    PortfolioBuilder::default()
        .id(node.id().to_string().try_into()?)
        .status(node.status().clone().into())
        .title(node.title().to_string().try_into()?)
        .created_at(node.created_at().to_string().try_into()?)
        .sections(node.content().iter().map(content_elements_mapper).collect())
        .metatags(metatags_field_mapper(node.metatags()))
        .build()
        .map_err(|e| AppError::unexpected(e))
}

fn content_elements_mapper(content: &ContentField) -> PortfolioSection {
    match content {
        ContentField::PortfolioAboutMeParagraph(_) => about_me_paragraph_mapper(&content),
        ContentField::PortfolioResumeParagraph(_) => resume_paragraph_mapper(&content),
        ContentField::PortfolioProjectsParagraph(_) => projects_paragraph_mapper(&content),
        ContentField::PortfolioArticlesParagraph(_) => blog_paragraph_mapper(&content),
        _ => PortfolioSection::Unknown,
    }
}

fn blog_paragraph_mapper(p: &ContentField) -> PortfolioSection {
    if let ContentField::PortfolioArticlesParagraph(p) = p {
        return PortfolioSection::Blogs(
            BlogsBuilder::default()
                .id(p.id().to_string().try_into().unwrap())
                .title(strip_tags(p.title()).try_into().unwrap())
                .subtitle(strip_tags(p.subtitle()).try_into().unwrap())
                .text(strip_tags(p.text()).try_into().unwrap())
                .articles(vec![]) // empty at the beginning, will be filled later
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn about_me_paragraph_mapper(p: &ContentField) -> PortfolioSection {
    if let ContentField::PortfolioAboutMeParagraph(p) = p {
        return PortfolioSection::AboutMe(
            AboutMeBuilder::default()
                .id(p.id().to_string().try_into().unwrap())
                .title(strip_tags(p.title()).try_into().unwrap())
                .subtitle(strip_tags(p.subtitle()).try_into().unwrap())
                .text(strip_tags(p.text()).try_into().unwrap())
                .skills(p.text_list().to_vec())
                .years_of_experience(p.years_of_experience().clone())
                .profile_picture(image_field_mapper(p.image()))
                .cv_document(document_field_mapper(p.document()))
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn resume_paragraph_mapper(p: &ContentField) -> PortfolioSection {
    if let ContentField::PortfolioResumeParagraph(p) = p {
        let education = p
            .items()
            .iter()
            .nth(0)
            .map(timeline_paragraph_mapper)
            .unwrap();

        let experience = p
            .items()
            .iter()
            .nth(1)
            .map(timeline_paragraph_mapper)
            .unwrap();

        return PortfolioSection::Resume(
            ResumeBuilder::default()
                .id(p.id().to_string().try_into().unwrap())
                .title(strip_tags(p.title()).try_into().unwrap())
                .subtitle(strip_tags(p.subtitle()).try_into().unwrap())
                .text(strip_tags(p.text()).try_into().unwrap())
                .education(education)
                .experience(experience)
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn projects_paragraph_mapper(p: &ContentField) -> PortfolioSection {
    if let ContentField::PortfolioProjectsParagraph(p) = p {
        let projects = p
            .items()
            .iter()
            .map(project_item_paragraph_mapper)
            .collect();

        return PortfolioSection::Projects(
            ProjectsBuilder::default()
                .id(p.id().to_string().try_into().unwrap())
                .title(strip_tags(p.title()).try_into().unwrap())
                .subtitle(strip_tags(p.subtitle()).try_into().unwrap())
                .text(strip_tags(p.text()).try_into().unwrap())
                .projects(projects)
                .build()
                .unwrap(),
        );
    }
    PortfolioSection::Unknown
}

fn image_field_mapper(p: &ImageField) -> Image {
    let url = p
        .media_image()
        .image_style_uri()
        .medium_500x500()
        .to_string()
        .try_into()
        .unwrap();

    ImageBuilder::default()
        .id(p.id().to_string().try_into().unwrap())
        .title(p.media_image().meta().alt().to_string().try_into().unwrap())
        .alt(p.media_image().meta().alt().to_string().try_into().unwrap())
        .height(p.media_image().meta().height().clone())
        .width(p.media_image().meta().width().clone())
        .url(url)
        .build()
        .unwrap()
}

fn document_field_mapper(p: &DocumentField) -> Document {
    let url = p
        .media_document()
        .uri()
        .url()
        .to_string()
        .try_into()
        .unwrap();

    let mimetype = p
        .media_document()
        .mime_type()
        .to_string()
        .try_into()
        .unwrap();

    DocumentBuilder::default()
        .id(p.id().to_string().try_into().unwrap())
        .url(url)
        .mime(mimetype)
        .build()
        .unwrap()
}

fn timeline_paragraph_mapper(s: &ContentField) -> Timeline {
    if let ContentField::ContentTimelineParagraph(p) = s {
        return TimelineBuilder::default()
            .id(p.id().to_string().try_into().unwrap())
            .items(
                p.items()
                    .iter()
                    .map(timeline_item_paragraph_mapper)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
            .build()
            .unwrap();
    }
    TimelineBuilder::default().build().unwrap()
}

fn timeline_item_paragraph_mapper(s: &ContentTimelineItemParagraph) -> TimelineItem {
    TimelineItemBuilder::default()
        .id(s.id().to_string().try_into().unwrap())
        .date(s.date4human().to_string().try_into().unwrap())
        .title(s.title().to_string().try_into().unwrap())
        .subtitle(s.subtitle().to_string().try_into().unwrap())
        .text(s.text().clone())
        .build()
        .unwrap()
}

fn project_item_paragraph_mapper(s: &ContentField) -> Project {
    if let ContentField::ContentProjectParagraph(p) = s {
        let link = LinkBuilder::default()
            .url(p.link().uri().to_string().try_into().unwrap())
            .title(p.link().title().to_string().try_into().unwrap())
            .options(vec![])
            .build()
            .unwrap();

        return ProjectBuilder::default()
            .id(p.id().to_string().try_into().unwrap())
            .title(strip_tags(p.title()).try_into().unwrap())
            .text(strip_tags(p.text().clone().unwrap_or_default().as_str()))
            .link(link)
            .image(image_field_mapper(p.media()))
            .build()
            .unwrap();
    }
    ProjectBuilder::default().build().unwrap()
}
