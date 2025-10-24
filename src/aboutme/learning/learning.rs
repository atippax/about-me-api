use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Learning {
    title: String,
    link: Option<String>,
    description: String,
}
pub fn learning() -> Vec<Learning> {
    vec![
        Learning {
            title: String::from("Problem Solving Zero To One"),
            link: None,
            description: String::from(
                "The Course is teach about how to solve problem and learn about how to coding like human communicate teach by dev rawitat Duration 8 day",
            ),
        },
        Learning {
            title: String::from("Rust mastery couse"),
            link: None,
            description: String::from(
                "The Course is teach rust teach by rustacern boy (dancing with my code)",
            ),
        },
    ]
}
