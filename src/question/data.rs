use super::types::*;
use eframe::egui::{DragValue, TextEdit, Ui};

#[derive(Clone, Debug)]
pub enum QuestionData {
    ShortAnswer(ShortAnswer),
    Paragraph(Paragraph),
    MultipleChoice(MultipleChoice),
    Checkboxes(Checkboxes),
    Dropdown(Dropdown),
    LinearScale(LinearScale),
    MultipleChoiceGrid(MultipleChoiceGrid),
    CheckboxGrid(CheckboxGrid),
    Date(Date),
    Time(Time),
}

impl QuestionData {
    pub fn name(&self) -> String {
        match self {
            Self::ShortAnswer(_) => "Short answer".into(),
            Self::Paragraph(_) => "Paragraph".into(),
            Self::MultipleChoice(_) => "Multiple choice".into(),
            Self::Checkboxes(_) => "Checkboxes".into(),
            Self::Dropdown(_) => "Dropdown".into(),
            Self::LinearScale(_) => "Linear scale".into(),
            Self::MultipleChoiceGrid(_) => "Multiple choice grid".into(),
            Self::CheckboxGrid(_) => "Checkbox grid".into(),
            Self::Date(_) => "Date".into(),
            Self::Time(_) => "Time".into(),
        }
    }

    pub fn types_list() -> Vec<Self> {
        vec![
            Self::ShortAnswer(ShortAnswer::default()),
            Self::Paragraph(Paragraph::default()),
            Self::MultipleChoice(MultipleChoice::default()),
            Self::Checkboxes(Checkboxes::default()),
            Self::Dropdown(Dropdown::default()),
            Self::LinearScale(LinearScale::default()),
            Self::MultipleChoiceGrid(MultipleChoiceGrid::default()),
            Self::CheckboxGrid(CheckboxGrid::default()),
            Self::Date(Date::default()),
            Self::Time(Time::default()),
        ]
    }

    pub fn edit(&mut self, ui: &mut Ui) {
        match self {
            Self::ShortAnswer(_) => {}
            Self::Paragraph(_) => {}
            Self::MultipleChoice(data) => edit_options(ui, &mut data.options),
            Self::Checkboxes(data) => edit_options(ui, &mut data.options),
            Self::Dropdown(data) => edit_options(ui, &mut data.options),
            Self::LinearScale(data) => {
                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut data.start).clamp_range(0..=1));
                    ui.label("to");
                    ui.add(DragValue::new(&mut data.end).clamp_range(2..=10));
                });

                ui.horizontal(|ui| {
                    ui.label(format!("{}", data.start));
                    ui.add(
                        TextEdit::singleline(&mut data.start_label)
                            .hint_text("Label (optional)")
                            .desired_width(100.0),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(format!("{}", data.end));
                    ui.add(
                        TextEdit::singleline(&mut data.end_label)
                            .hint_text("Label (optional)")
                            .desired_width(100.0),
                    );
                });
            }
            Self::MultipleChoiceGrid(data) => edit_grid(ui, &mut data.rows, &mut data.columns),
            Self::CheckboxGrid(data) => edit_grid(ui, &mut data.rows, &mut data.columns),
            Self::Date(_) => {}
            Self::Time(_) => {}
        }
    }
}

pub fn edit_grid(ui: &mut Ui, rows: &mut Vec<String>, columns: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| edit_options(ui, rows));
        ui.vertical(|ui| edit_options(ui, columns));
    });
}

pub fn edit_options(ui: &mut Ui, options: &mut Vec<String>) {
    let mut delete_option = None;
    for (i, option) in options.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(option).desired_width(100.0));
            if ui.button("Delete").clicked() {
                delete_option = Some(i);
            }
        });
    }
    if let Some(i) = delete_option {
        options.remove(i);
    }
    if ui.button("Add option").clicked() {
        options.push("Option".into());
    }
}
