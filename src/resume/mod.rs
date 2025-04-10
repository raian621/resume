pub mod reader;
pub mod render;

pub mod resume {
    use std::io::Read;

    use serde::{Deserialize, Serialize};

    use super::render;
    pub use render::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Resume {
        pub contact: Contact,
        pub experience: Option<Vec<Experience>>,
        pub education: Option<Vec<Education>>,
        pub certifications: Option<Vec<Certification>>,
        pub projects: Option<Vec<Project>>,
        pub skills: Option<Skills>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Contact {
        pub name: String,
        pub email: Option<String>,
        pub phone: Option<String>,
        pub address: Option<String>,
        pub github: Option<String>,
        pub linkedin: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Experience {
        pub title: String,
        pub company: String,
        pub location: String,
        pub start_date: String,
        pub end_date: String,
        pub highlights: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Education {
        pub school: String,
        pub degree: String,
        pub start_date: String,
        pub end_date: String,
        pub location: String,
        pub highlights: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Certification {
        pub title: String,
        pub issuer: String,
        pub date: String,
        pub expires: String,
        pub url: String,
        pub highlights: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Project {
        pub title: String,
        pub technologies: Option<Vec<String>>,
        pub highlights: Option<Vec<String>>,
        pub source_link: Option<String>,
        pub demo_link: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    pub struct Skills {
        pub languages: Option<Vec<String>>,
        pub frameworks: Option<Vec<String>>,
        pub databases: Option<Vec<String>>,
        pub tools: Option<Vec<String>>,
    }

    pub fn read_resume_from_input<R: Read>(input: R) -> serde_yaml::Result<Resume> {
        serde_yaml::from_reader(input)
    }
}
