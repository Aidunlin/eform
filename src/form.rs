use super::question::*;

#[derive(Clone, Debug)]
pub struct Form {
    pub name: String,
    pub questions: Vec<Question>,
}

impl Form {
    pub fn new() -> Self {
        Self {
            name: "Untitled form".into(),
            questions: vec![],
        }
    }
}
