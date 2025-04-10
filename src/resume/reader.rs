use std::io::Read;

use super::resume::Resume;

pub fn read_resume_from_input<R: Read>(input: R) -> serde_yaml::Result<Resume> {
    serde_yaml::from_reader(input)
}

#[cfg(test)]
pub mod tests {
    use std::{io::Cursor, vec};

    use tera::Tera;

    use crate::resume::resume::{Certification, Contact, Education, Experience, Project, Skills};

    use super::*;

    fn get_contact() -> Contact {
        Contact {
            name: "John Doe".to_owned(),
            email: Some("johndoe@gmail.com".to_owned()),
            phone: Some("123-456-7890".to_owned()),
            address: Some("1234 Street Avenue".to_owned()),
            github: Some("github.com/johndoe".to_owned()),
            linkedin: Some("linkedin.com/in/johndoe".to_owned()),
        }
    }

    fn get_experience_1() -> Experience {
        Experience {
            title: "Software Engineer".to_owned(),
            company: "Acme Inc.".to_owned(),
            location: "San Francisco, CA".to_owned(),
            start_date: "2018-01-01".to_owned(),
            end_date: "2022-12-31".to_owned(),
            highlights: Some(vec![
                "Worked on a team of developers to develop a new feature for a web application."
                    .to_owned(),
                "Led a project to implement a new feature for a mobile app.".to_owned(),
            ]),
        }
    }

    fn get_experience_2() -> Experience {
        Experience {
            title: "Senior Software Engineer".to_owned(),
            company: "Google".to_owned(),
            location: "Mountain View, CA".to_owned(),
            start_date: "2023-01-01".to_owned(),
            end_date: "present".to_owned(),
            highlights: Some(vec![
                "Worked on a team of developers to develop a new feature for a web application."
                    .to_owned(),
                "Led a project to implement a new feature for a mobile app.".to_owned(),
            ]),
        }
    }

    fn get_education_1() -> Education {
        Education {
            school: "University of California".to_owned(),
            degree: "Bachelor of Science in Computer Science".to_owned(),
            start_date: "2014-09-01".to_owned(),
            end_date: "2018-06-30".to_owned(),
            location: "San Diego".to_owned(),
            highlights: Some(vec![
                "Graduated with honors.".to_owned(),
                "Received a scholarship.".to_owned(),
            ]),
        }
    }

    fn get_education_2() -> Education {
        Education {
            school: "University of California".to_owned(),
            degree: "Master of Science in Computer Science".to_owned(),
            start_date: "2018-09-01".to_owned(),
            end_date: "2022-06-30".to_owned(),
            location: "San Diego".to_owned(),
            highlights: Some(vec![
                "Graduated with honors.".to_owned(),
                "Received a scholarship.".to_owned(),
            ]),
        }
    }

    fn get_certification_1() -> Certification {
        Certification {
            title: "AWS Certified Cloud Practitioner".to_owned(),
            issuer: "Amazon Web Services".to_owned(),
            date: "2023-01-01".to_owned(),
            expires: "2023-12-31".to_owned(),
            url: "https://aws.amazon.com/certification".to_owned(),
            highlights: Some(vec![
                "Learned how to use AWS services.".to_owned(),
                "Completed a hands-on lab.".to_owned(),
            ]),
        }
    }

    fn get_certification_2() -> Certification {
        Certification {
            title: "Azure Developer Associate".to_owned(),
            issuer: "Microsoft".to_owned(),
            date: "2023-01-01".to_owned(),
            expires: "2023-12-31".to_owned(),
            url: "https://azure.microsoft.com/en-us/certification/developer-associate".to_owned(),
            highlights: Some(vec![
                "Learned how to use Azure services.".to_owned(),
                "Completed a hands-on lab.".to_owned(),
            ]),
        }
    }

    fn get_project_1() -> Project {
        Project {
            title: "Project 1".to_owned(),
            technologies: Some(vec!["Python".to_owned(), "AWS".to_owned()]),
            source_link: Some("https://github.com/johndoe/project1".to_owned()),
            demo_link: Some("https://example.com".to_owned()),
            highlights: Some(vec![
                "Learned how to use AWS services.".to_owned(),
                "Completed a hands-on lab.".to_owned(),
            ]),
        }
    }

    fn get_skills() -> Skills {
        Skills {
            languages: Some(vec![
                "Java".to_owned(),
                "Python".to_owned(),
                "C/C++".to_owned(),
                "SQL (Postgres)".to_owned(),
                "JavaScript".to_owned(),
                "HTML/CSS".to_owned(),
                "R".to_owned(),
            ]),
            frameworks: Some(vec![
                "React".to_owned(),
                "Node.js".to_owned(),
                "Flask".to_owned(),
                "JUnit".to_owned(),
                "WordPress".to_owned(),
                "Material-UI".to_owned(),
                "FastAPI".to_owned(),
            ]),
            databases: Some(vec!["PostgreSQL".to_owned(), "MongoDB".to_owned()]),
            tools: Some(vec!["Ansible".to_owned(), "Docker".to_owned()]),
        }
    }

    fn get_project_2() -> Project {
        Project {
            title: "Project 2".to_owned(),
            technologies: Some(vec!["Python".to_owned(), "Azure".to_owned()]),
            source_link: Some("https://github.com/johndoe/project2".to_owned()),
            demo_link: Some("https://example.com".to_owned()),
            highlights: Some(vec![
                "Learned how to use Azure services.".to_owned(),
                "Completed a hands-on lab.".to_owned(),
            ]),
        }
    }

    fn get_resume_from_template(template_name: &str) -> Resume {
        let tera = Tera::new("test/templates/**/*").unwrap();
        let resume_yaml = tera.render(template_name, &tera::Context::new());
        read_resume_from_input(Cursor::new(resume_yaml.unwrap())).unwrap()
    }

    #[test]
    fn test_read_full_resume_from_input() {
        let expected = Resume {
            contact: get_contact(),
            experience: Some(vec![get_experience_1(), get_experience_2()]),
            education: Some(vec![get_education_1(), get_education_2()]),
            certifications: Some(vec![get_certification_1(), get_certification_2()]),
            projects: Some(vec![get_project_1(), get_project_2()]),
            skills: Some(get_skills()),
        };
        assert_eq!(get_resume_from_template("full-resume.tmpl.yml"), expected);
    }

    #[test]
    fn test_read_bare_resume_from_input() {
        let expected = Resume {
            contact: get_contact(),
            experience: None,
            education: None,
            certifications: None,
            projects: None,
            skills: None,
        };
        assert_eq!(get_resume_from_template("bare-resume.tmpl.yml"), expected);
    }
}
