use std::{fs, path::Path};

use indexmap::IndexMap;

use anyhow::Result;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use once_cell::sync::Lazy;
use tera::Context;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "templates/"]
#[include = "*.tmpl"]
struct Asset;

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    let template_list = ["resume.tex.tmpl", "resume.md.tmpl"];
    for each_temp in template_list {
        if let Some(content) = Asset::get(each_temp) {
            tera.add_raw_template(
                each_temp,
                std::str::from_utf8(content.data.as_ref()).expect("Internal Error (See templates)"),
            )
            .expect("Tera error adding templates");
        }
    }
    tera
});

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CVStruct {
    pub meta: Option<Vec<String>>,
    pub header: Option<Header>,
    pub education: Option<Vec<EducationField>>,
    pub experience: Option<Vec<ExperienceField>>,
    pub projects: Option<Vec<ProjectField>>,
    pub skills: Option<IndexMap<String, String>>,
}

pub enum OutputType {
    Tex,
    HTML,
    Markdown,
    TOML,
}

impl CVStruct {
    pub fn get_dummy() -> CVStruct {
        CVStruct {
            meta: None,
            header: Some(Header {
                full_name: "John Doe".to_string(),
                email_addr: "john@doe.com".to_string(),
                github_username: "github".to_string(),
                linkedin_username: "linkedin".to_string(),
                location: "Some Place, Earth".to_string(),
                phone_number: "1234567890".to_string(),
            }),
            education: Some(vec![EducationField {
                course_name: "Bachelor of Technology in Computer Science".to_string(),
                timeline: "August 2003 - July 2007".to_string(),
                university_name: "University at My Place".to_string(),
                location: "Another Place, Earth".to_string(),
                course_grade: "A".to_string(),
                points: Some(vec![
                    "Did some useful work Y".to_string(),
                    "Did some useful work Z".to_string(),
                ]),
                university_link: "https://myopenuniversity.university".to_string(),
            }]),
            experience: Some(vec![ExperienceField {
                comp_name: "Company ABC".to_string(),
                exp_name: "Engineer".to_string(),
                timeline: "August 2008 - Present".to_string(),
                location: "Another place, Earth".to_string(),
                points: Some(vec![
                    "I joined this when i had mood to join".to_string(),
                    "I joined this to prove myself".to_string(),
                    "I achieved ABC here".to_string(),
                ]),
                comp_link: "https://mygoodcompany.company".to_string(),
            }]),
            projects: Some(vec![ProjectField {
                title: "My good project XYZ".to_string(),
                timeline: Some("January 2006 - Present".to_string()),
                project_link: "https://github.com/github".to_string(),
                description: "Some incredible project that I created".to_string(),
                points: Some(vec![
                    "I created this when i had mood to create".to_string(),
                    "I created this to prove myself".to_string(),
                ]),
            }]),
            skills: Some(IndexMap::from([
                (
                    "Technologies".to_string(),
                    "A, B, C, D, E, F, G, H, whatever".to_string(),
                ),
                (
                    "Frameworks".to_string(),
                    "ABC, DEF, GHI, JKL, whatever".to_string(),
                ),
            ])),
        }
    }

    pub fn from_toml_file(filename: &impl AsRef<Path>) -> Result<CVStruct> {
        let data: String = fs::read_to_string(filename)?;
        let cv: CVStruct = toml::from_str(&data)?;
        Ok(cv)
    }

    pub fn from_toml(content: &str) -> Result<CVStruct> {
        let cv: CVStruct = toml::from_str(&content)?;
        Ok(cv)
    }

    pub fn to_toml(&self) -> Result<String> {
        let resume_toml = toml::to_string(&self)?;
        Ok(resume_toml)
    }

    pub fn to_tex(&self) -> Result<String> {
        let mut context = Context::new();
        context.insert("cvstruct", &self);

        let resume_tex = TEMPLATES.render("resume.tex.tmpl", &context)?;
        Ok(resume_tex)
    }

    pub fn to_md(&self) -> Result<String> {
        let mut context = Context::new();
        context.insert("cvstruct", &self);
        let resume_md = TEMPLATES.render("resume.md.tmpl", &context)?;
        Ok(resume_md)
    }

    pub fn to_html(&self) -> Result<String> {
        let mut context = Context::new();
        context.insert("cvstruct", &self);
        let resume_html = TEMPLATES.render("resume.html.tmpl", &context)?;
        Ok(resume_html)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Header {
    pub full_name: String,
    pub email_addr: String,
    pub github_username: String,
    pub linkedin_username: String,
    pub location: String,
    pub phone_number: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EducationField {
    pub course_name: String,
    pub timeline: String,
    pub university_name: String,
    pub university_link: String,
    pub location: String,
    pub course_grade: String,
    pub points: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExperienceField {
    pub comp_name: String,
    pub comp_link: String,
    pub exp_name: String,
    pub timeline: String,
    pub location: String,
    pub points: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectField {
    pub title: String,
    pub timeline: Option<String>,
    pub project_link: String,
    pub description: String,
    pub points: Option<Vec<String>>,
}
