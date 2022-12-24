use super::value_types;

#[derive(Clone, Debug)]
pub enum QuestionValue {
    ShortAnswer(value_types::ShortAnswerValue),
    Paragraph(value_types::ParagraphValue),
    MultipleChoice(value_types::MultipleChoiceValue),
    Checkboxes(value_types::CheckboxesValue),
    Dropdown(value_types::DropdownValue),
    LinearScale(value_types::LinearScaleValue),
    MultipleChoiceGrid(value_types::MultipleChoiceGridValue),
    CheckboxGrid(value_types::CheckboxGridValue),
    Date(value_types::DateValue),
    Time(value_types::TimeValue),
}

impl QuestionValue {
    pub fn types_list() -> Vec<Self> {
        vec![
            Self::ShortAnswer(value_types::ShortAnswerValue::default()),
            Self::Paragraph(value_types::ParagraphValue::default()),
            Self::MultipleChoice(value_types::MultipleChoiceValue::default()),
            Self::Checkboxes(value_types::CheckboxesValue::default()),
            Self::Dropdown(value_types::DropdownValue::default()),
            Self::LinearScale(value_types::LinearScaleValue::default()),
            Self::MultipleChoiceGrid(value_types::MultipleChoiceGridValue::default()),
            Self::CheckboxGrid(value_types::CheckboxGridValue::default()),
            Self::Date(value_types::DateValue::default()),
            Self::Time(value_types::TimeValue::default()),
        ]
    }
}

impl PartialEq for QuestionValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ShortAnswer(_), Self::ShortAnswer(_)) => true,
            (Self::Paragraph(_), Self::Paragraph(_)) => true,
            (Self::MultipleChoice(_), Self::MultipleChoice(_)) => true,
            (Self::Checkboxes(_), Self::Checkboxes(_)) => true,
            (Self::Dropdown(_), Self::Dropdown(_)) => true,
            (Self::LinearScale(_), Self::LinearScale(_)) => true,
            (Self::MultipleChoiceGrid(_), Self::MultipleChoiceGrid(_)) => true,
            (Self::CheckboxGrid(_), Self::CheckboxGrid(_)) => true,
            (Self::Date(_), Self::Date(_)) => true,
            (Self::Time(_), Self::Time(_)) => true,
            _ => false,
        }
    }
}
