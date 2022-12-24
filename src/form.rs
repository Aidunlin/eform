#[derive(Clone, Debug)]
pub struct Form {
    pub name: String,
    pub questions: Vec<crate::question::Question>,
}

impl Form {
    pub fn new() -> Self {
        Self {
            name: "Untitled form".into(),
            questions: vec![],
        }
    }
}
