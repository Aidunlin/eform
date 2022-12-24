use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ShortAnswerValue {
    pub text: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParagraphValue {
    pub text: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MultipleChoiceValue {
    pub choice: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CheckboxesValue {
    pub choices: Vec<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DropdownValue {
    pub choice: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LinearScaleValue {
    pub value: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MultipleChoiceGridValue {
    pub choices: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CheckboxGridValue {
    pub choices: Vec<Vec<bool>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DayPeriod {
    AM,
    PM,
}

impl Default for DayPeriod {
    fn default() -> Self {
        Self::AM
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DateValue {
    pub year: usize,
    pub month: usize,
    pub day: usize,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TimeValue {
    pub hour: usize,
    pub minute: usize,
    pub period: DayPeriod,
}
