#[derive(Clone, Debug, Default)]
pub struct ShortAnswerConfig;

#[derive(Clone, Debug, Default)]
pub struct ParagraphConfig;

#[derive(Clone, Debug, Default)]
pub struct MultipleChoiceConfig {
    pub options: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CheckboxesConfig {
    pub options: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct DropdownConfig {
    pub options: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct LinearScaleScale {
    pub start: usize,
    pub start_label: String,
    pub end: usize,
    pub end_label: String,
}

impl Default for LinearScaleScale {
    fn default() -> Self {
        Self {
            start: 1,
            end: 5,
            start_label: String::new(),
            end_label: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MultipleChoiceGridConfig {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CheckboxGridConfig {
    pub rows: Vec<String>,
    pub columns: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct DateConfig;

#[derive(Clone, Debug, Default)]
pub struct TimeConfig;
