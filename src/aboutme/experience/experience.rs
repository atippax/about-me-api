use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct JobTechnology {
    job_title: String,
    link: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobExperience {
    company_name: String,
    start: String,
    end: String,
    job_role: String,
    technologies: Vec<JobTechnology>,
    responsibility: Vec<String>,
}
pub fn experiences() -> Vec<JobExperience> {
    vec![
        JobExperience {
            start: String::from(""),
            company_name: String::from("10x Plus"),
            end: String::from(""),
            responsibility: vec![
                String::from(
                    "Migrate Nuxt2 -> Nuxt3 And change a coding Style from option api to composition api",
                ),
                String::from(
                    "Create a library to store a global constaint use in frontnend and backend ",
                ),
                String::from("Deployment and moniter"),
            ],
            job_role: String::from("Developer Officer (Full-Stack)"),
            technologies: vec![
                JobTechnology {
                    job_title: String::from("Nuxt3"),
                    link: None,
                },
                JobTechnology {
                    job_title: String::from("Nest"),
                    link: None,
                },
            ],
        },
        JobExperience {
            start: String::from(""),
            company_name: String::from("Clicknext"),
            end: String::from(""),
            responsibility: vec![String::from("Create a CRUD Api with ASP.NET c#")],
            job_role: String::from("Intern full-stack devloper"),
            technologies: vec![
                JobTechnology {
                    job_title: String::from("Vue3"),
                    link: None,
                },
                JobTechnology {
                    job_title: String::from("C# (ASP.NET)"),
                    link: None,
                },
            ],
        },
        JobExperience {
            start: String::from(""),
            company_name: String::from("Super dev"),
            end: String::from(""),
            responsibility: vec![String::from(
                "Developer a frontnend application each project",
            )],
            job_role: String::from(""),
            technologies: vec![],
        },
    ]
}
