use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Excited {
    title: String,
    link: Option<String>,
    description: String,
}

pub fn excited() -> Vec<Excited> {
    vec![
        Excited {
            description: String::from("Financial/Investment"),
            link: None,
            title: String::from("Financial/Investment"),
        },
        Excited {
            description: String::from("Longivity/healthy"),
            link: None,
            title: String::from("Longivity/healthy"),
        },
        Excited {
            description: String::from("Programing/Technologies"),
            link: None,
            title: String::from("Programing/Technologies"),
        },
        Excited {
            description: String::from("Philothophy/lifing"),
            link: None,
            title: String::from("Philothophy/lifing"),
        },
        Excited {
            description: String::from("Music"),
            link: None,
            title: String::from("Music"),
        },
        Excited {
            description: String::from("Historical"),
            link: None,
            title: String::from("Historical"),
        },
    ]
}
