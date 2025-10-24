use serde::Serialize;

use crate::aboutme::{
    excited::excited::{Excited, excited},
    experience::experience::{JobExperience, experiences},
    learning::learning::{Learning, learning},
};
#[derive(Debug, Clone, Serialize)]
pub struct Aboutme {
    first_name: String,
    last_name: String,
    exciteds: Vec<Excited>,
    experiences: Vec<JobExperience>,
    learnings: Vec<Learning>,
}
pub fn aboutme() -> Aboutme {
    Aboutme {
        first_name: String::from("Atip"),
        last_name: String::from("Nasakun"),
        exciteds: excited(),
        experiences: experiences(),
        learnings: learning(),
    }
}
