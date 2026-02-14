use derive_builder::Builder;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::application::domain::article::Articles;
use crate::application::domain::common::{Document, Image, MetaTags, Project, Timeline};
use crate::application::value_objects::{Date, Identifier, ModerationStatus, RequiredText};

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Portfolio {
    id: Identifier,
    title: RequiredText,
    created_at: Date,
    status: ModerationStatus,
    metatags: MetaTags,
    sections: Vec<PortfolioSection>,
}

impl Portfolio {
    pub fn sections_mut(&mut self) -> &mut Vec<PortfolioSection> {
        &mut self.sections
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortfolioSection {
    AboutMe(AboutMe),
    Resume(Resume),
    Projects(Projects),
    Blogs(Blogs),
    Unknown,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Resume {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: RequiredText,
    education: Timeline,
    experience: Timeline,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct AboutMe {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: RequiredText,
    skills: Vec<String>,
    cv_document: Document,
    profile_picture: Image,
    years_of_experience: u8,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Projects {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: RequiredText,
    projects: Vec<Project>,
}

#[derive(Debug, Clone, Getters, Serialize, Deserialize, Builder)]
pub struct Blogs {
    id: Identifier,
    title: RequiredText,
    subtitle: RequiredText,
    text: String,
    articles: Articles,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::application::domain::article::tests::article_fixture;
    use crate::application::domain::common::tests::{
        document_fixture, image_fixture, metatags_fixture, project_fixture, timeline_fixture,
    };

    #[test]
    fn creation_succeeds_when_valid_portfolio() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        portfolio_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_resume_section() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        resume_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_about_me_section() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        about_me_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_projects_section() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        projects_fixture();
    }

    #[test]
    fn creation_succeeds_when_valid_blogs_sections() {
        // the fixture calls the builder methods, so we just ensure that it doesn't panic
        blogs_fixture();
    }

    #[test]
    fn serialization_succeeds_when_valid_portfolio() {
        let p = portfolio_fixture();
        let serialized = serde_json::json!(&p).to_string();

        assert!(serialized.contains(p.id().to_string().as_str()));
        assert!(serialized.contains(p.title().as_str()));
        assert!(serialized.contains(p.status().to_string().as_str()));
    }

    #[test]
    fn deserialization_succeeds_when_valid_portfolio() {
        let p = portfolio_fixture();
        let serialized = serde_json::json!(&p).to_string();
        let deserialized: Portfolio = serde_json::from_str(&serialized).unwrap();

        assert_eq!(p.id(), deserialized.id());
        assert_eq!(p.title(), deserialized.title());
        assert_eq!(p.status(), deserialized.status());
        assert_eq!(p.sections().len(), deserialized.sections().len());
    }

    pub fn portfolio_fixture() -> Portfolio {
        PortfolioBuilder::default()
            .id(Identifier::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap())
            .title(RequiredText::try_from("Portfolio John Doe").unwrap())
            .status(ModerationStatus::Published)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .sections(sections_fixture())
            .metatags(metatags_fixture())
            .build()
            .unwrap()
    }

    pub fn unpublished_portfolio_fixture() -> Portfolio {
        PortfolioBuilder::default()
            .id(Identifier::try_from("550e8400-e29b-41d4-a716-446655440000").unwrap())
            .title(RequiredText::try_from("Portfolio John Doe").unwrap())
            .status(ModerationStatus::Unpublished)
            .created_at(Date::try_from("2024-12-15T14:03:56+00:00").unwrap())
            .sections(sections_fixture())
            .metatags(metatags_fixture())
            .build()
            .unwrap()
    }

    fn sections_fixture() -> Vec<PortfolioSection> {
        vec![
            PortfolioSection::Resume(resume_fixture()),
            PortfolioSection::AboutMe(about_me_fixture()),
            PortfolioSection::Projects(projects_fixture()),
            PortfolioSection::Blogs(blogs_fixture()),
        ]
    }

    #[test]
    fn serialization_succeeds_when_valid_resume_section() {
        let r = resume_fixture();
        let serialized = serde_json::to_string(&r).unwrap();

        assert!(serialized.contains(r.id().to_string().as_str()));
        assert!(serialized.contains(r.title().as_str()));
        assert!(serialized.contains(r.text().as_str()));
        assert!(serialized.contains(r.subtitle().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_about_me_section() {
        let a = about_me_fixture();
        let serialized = serde_json::to_string(&a).unwrap();

        assert!(serialized.contains(a.id().to_string().as_str()));
        assert!(serialized.contains(a.title().as_str()));
        assert!(serialized.contains(a.subtitle().as_str()));
        assert!(serialized.contains(a.text().as_str()));
        assert!(serialized.contains(a.skills().get(0).unwrap().as_str()));
        assert!(serialized.contains(a.years_of_experience().to_string().as_str()));
        assert!(serialized.contains(a.cv_document().id().to_string().as_str()));
        assert!(serialized.contains(a.cv_document().url().as_str()));
        assert!(serialized.contains(a.cv_document().mime().as_str()));
        assert!(serialized.contains(a.profile_picture().id().to_string().as_str()));
        assert!(serialized.contains(a.profile_picture().url().as_str()));
        assert!(serialized.contains(a.profile_picture().alt().as_str()));
        assert!(serialized.contains(a.profile_picture().width().to_string().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_projects_section() {
        let p = projects_fixture();
        let first_project = p.projects().get(0).unwrap();
        let serialized = serde_json::to_string(&p).unwrap();

        assert!(serialized.contains(p.id().to_string().as_str()));
        assert!(serialized.contains(p.title().as_str()));
        assert!(serialized.contains(p.text().as_str()));
        assert!(serialized.contains(p.subtitle().as_str()));
        assert!(serialized.contains(first_project.id().to_string().as_str()));
        assert!(serialized.contains(first_project.title().as_str()));
        assert!(serialized.contains(first_project.text().as_str()));
        assert!(serialized.contains(first_project.image().url().as_str()));
        assert!(serialized.contains(first_project.image().alt().as_str()));
        assert!(serialized.contains(first_project.image().title().as_str()));
        assert!(serialized.contains(first_project.image().width().to_string().as_str()));
        assert!(serialized.contains(first_project.link().url().as_str()));
        assert!(serialized.contains(first_project.link().title().as_str()));
    }

    #[test]
    fn serialization_succeeds_when_valid_blogs_sections() {
        let b = blogs_fixture();
        let first_blog = b.articles().get(0).unwrap();
        let serialized = serde_json::to_string(&b).unwrap();

        assert!(serialized.contains(b.id().to_string().as_str()));
        assert!(serialized.contains(b.title().as_str()));
        assert!(serialized.contains(b.text().as_str()));
        assert!(serialized.contains(b.subtitle().as_str()));
        assert!(serialized.contains(first_blog.id().to_string().as_str()));
        assert!(serialized.contains(first_blog.slug().as_str()));
        assert!(serialized.contains(first_blog.title().as_str()));
        assert!(serialized.contains(first_blog.summary().as_str()));
        assert!(serialized.contains(first_blog.thumbnail().url().as_str()));
        assert!(serialized.contains(first_blog.thumbnail().alt().as_str()));
        assert!(serialized.contains(first_blog.thumbnail().title().as_str()));
        assert!(serialized.contains(first_blog.thumbnail().width().to_string().as_str()));
        assert!(serialized.contains(first_blog.thumbnail().height().to_string().as_str()));
        assert!(serialized.contains(first_blog.category().title().as_str()));
        assert!(serialized.contains(first_blog.category().emoji().as_str()));
    }

    #[test]
    fn deserialization_succeeds_when_valid_resume_section() {
        let r = resume_fixture();
        let serialized = serde_json::json!(&r).to_string();
        let deserialized: Resume = serde_json::from_str(&serialized).unwrap();

        assert_eq!(r.id(), deserialized.id());
        assert_eq!(r.title(), deserialized.title());
        assert_eq!(r.subtitle(), deserialized.subtitle());
        assert_eq!(r.text(), deserialized.text());
    }

    #[test]
    fn deserialization_succeeds_when_valid_about_me_section() {
        let a = about_me_fixture();
        let picture = a.profile_picture();
        let serialized = serde_json::json!(&a).to_string();
        let deserialized: AboutMe = serde_json::from_str(&serialized).unwrap();

        assert_eq!(a.id(), deserialized.id());
        assert_eq!(a.title(), deserialized.title());
        assert_eq!(a.subtitle(), deserialized.subtitle());
        assert_eq!(a.text(), deserialized.text());
        assert_eq!(a.skills(), deserialized.skills());
        assert_eq!(a.years_of_experience(), deserialized.years_of_experience());
        assert_eq!(a.cv_document().id(), deserialized.cv_document().id());
        assert_eq!(a.cv_document().url(), deserialized.cv_document().url());
        assert_eq!(a.cv_document().mime(), deserialized.cv_document().mime());
        assert_eq!(picture.id(), deserialized.profile_picture().id());
        assert_eq!(picture.url(), deserialized.profile_picture().url());
        assert_eq!(picture.alt(), deserialized.profile_picture().alt());
        assert_eq!(picture.width(), deserialized.profile_picture().width());
    }

    #[test]
    fn deserialization_succeeds_when_valid_projects_section() {
        let p = projects_fixture();
        let serialized = serde_json::json!(&p).to_string();
        let deserialized: Projects = serde_json::from_str(&serialized).unwrap();

        assert_eq!(p.id(), deserialized.id());
        assert_eq!(p.title(), deserialized.title());
        assert_eq!(p.subtitle(), deserialized.subtitle());
        assert_eq!(p.text(), deserialized.text());
        assert_eq!(p.projects().len(), deserialized.projects().len());
    }

    #[test]
    fn deserialization_succeeds_for_blogs() {
        let b = blogs_fixture();
        let serialized = serde_json::json!(&b).to_string();
        let deserialized: Blogs = serde_json::from_str(&serialized).unwrap();

        assert_eq!(b.id(), deserialized.id());
        assert_eq!(b.title(), deserialized.title());
        assert_eq!(b.subtitle(), deserialized.subtitle());
        assert_eq!(b.text(), deserialized.text());
        assert_eq!(b.articles().len(), deserialized.articles().len());
    }

    pub fn about_me_fixture() -> AboutMe {
        AboutMeBuilder::default()
            .id("413b8ba1-2bc4-4fda-8455-0c0dea528ca0".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text("Example text".try_into().unwrap())
            .skills(vec!["Example skill 1".to_string()])
            .cv_document(document_fixture())
            .profile_picture(image_fixture())
            .years_of_experience(5)
            .build()
            .unwrap()
    }

    pub fn resume_fixture() -> Resume {
        ResumeBuilder::default()
            .id("523b8ba1-2bc4-4fda-8455-0c0dea528ca1".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text("Example text".try_into().unwrap())
            .education(timeline_fixture())
            .experience(timeline_fixture())
            .build()
            .unwrap()
    }

    pub fn projects_fixture() -> Projects {
        ProjectsBuilder::default()
            .id("633b8ba1-2bc4-4fda-8455-0c0dea528ca2".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .text("Example text".try_into().unwrap())
            .projects(vec![project_fixture(), project_fixture()])
            .build()
            .unwrap()
    }

    pub fn blogs_fixture() -> Blogs {
        BlogsBuilder::default()
            .id("743b8ba1-2bc4-4fda-8455-0c0dea528ca3".try_into().unwrap())
            .title("Example title".try_into().unwrap())
            .subtitle("Example subtitle".try_into().unwrap())
            .articles(vec![article_fixture(), article_fixture()])
            .text("Example text".to_string())
            .build()
            .unwrap()
    }
}
