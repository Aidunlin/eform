#[derive(Clone, Debug, Default)]
pub struct ShortAnswer {
    pub text: String,
}

#[derive(Clone, Debug, Default)]
pub struct Paragraph {
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct MultipleChoice {
    pub options: Vec<String>,
    pub choice: String,
}

impl Default for MultipleChoice {
    fn default() -> Self {
        Self {
            options: vec!["Option 1".into()],
            choice: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Checkboxes {
    pub options: Vec<String>,
    pub choices: Vec<bool>,
}

impl Default for Checkboxes {
    fn default() -> Self {
        Self {
            options: vec!["Option 1".into()],
            choices: vec![false],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Dropdown {
    pub options: Vec<String>,
    pub choice: String,
}

impl Default for Dropdown {
    fn default() -> Self {
        Self {
            options: vec!["Option 1".into()],
            choice: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinearScale {
    pub start: usize,
    pub start_label: String,
    pub end: usize,
    pub end_label: String,
    pub value: usize,
}

impl Default for LinearScale {
    fn default() -> Self {
        Self {
            start: 1,
            end: 5,
            start_label: String::new(),
            end_label: String::new(),
            value: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MultipleChoiceGrid {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    pub choices: Vec<String>,
}

impl Default for MultipleChoiceGrid {
    fn default() -> Self {
        Self {
            rows: vec!["Row 1".into()],
            columns: vec!["Column 1".into()],
            choices: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckboxGrid {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
    pub choices: Vec<Vec<bool>>,
}

impl Default for CheckboxGrid {
    fn default() -> Self {
        Self {
            rows: vec!["Row 1".into()],
            columns: vec!["Column 1".into()],
            choices: vec![vec![false]],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DayPeriod {
    AM,
    PM,
}

impl Default for DayPeriod {
    fn default() -> Self {
        Self::AM
    }
}

#[derive(Clone, Debug, Default)]
pub struct Date {
    pub year: usize,
    pub month: usize,
    pub day: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Time {
    pub hour: usize,
    pub minute: usize,
    pub period: DayPeriod,
}
