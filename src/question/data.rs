use eframe::egui;

use super::types;

#[derive(Clone, Debug)]
pub enum QuestionData {
    ShortAnswer(types::ShortAnswer),
    Paragraph(types::Paragraph),
    MultipleChoice(types::MultipleChoice),
    Checkboxes(types::Checkboxes),
    Dropdown(types::Dropdown),
    LinearScale(types::LinearScale),
    MultipleChoiceGrid(types::MultipleChoiceGrid),
    CheckboxGrid(types::CheckboxGrid),
    Date(types::Date),
    Time(types::Time),
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
            Self::ShortAnswer(types::ShortAnswer::default()),
            Self::Paragraph(types::Paragraph::default()),
            Self::MultipleChoice(types::MultipleChoice::default()),
            Self::Checkboxes(types::Checkboxes::default()),
            Self::Dropdown(types::Dropdown::default()),
            Self::LinearScale(types::LinearScale::default()),
            Self::MultipleChoiceGrid(types::MultipleChoiceGrid::default()),
            Self::CheckboxGrid(types::CheckboxGrid::default()),
            Self::Date(types::Date::default()),
            Self::Time(types::Time::default()),
        ]
    }

    pub fn reset_value(&mut self) {
        match self {
            Self::ShortAnswer(data) => {
                data.text = String::new();
            }
            Self::Paragraph(data) => {
                data.text = String::new();
            }
            Self::MultipleChoice(data) => {
                data.choice = String::new();
            }
            Self::Checkboxes(data) => {
                data.choices = vec![false; data.options.len()];
            }
            Self::Dropdown(data) => {
                data.choice = String::new();
            }
            Self::LinearScale(data) => {
                data.value = data.start;
            }
            Self::MultipleChoiceGrid(data) => {
                data.choices = vec![String::new(); data.rows.len()];
            }
            Self::CheckboxGrid(data) => {
                data.choices = vec![vec![false; data.columns.len()]; data.rows.len()];
            }
            Self::Date(data) => {
                data.year = 0;
                data.month = 1;
                data.day = 1;
            }
            Self::Time(data) => {
                data.hour = 1;
                data.minute = 0;
                data.period = types::DayPeriod::AM;
            }
        }
    }

    pub fn edit(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::ShortAnswer(_) => {}
            Self::Paragraph(_) => {}
            Self::MultipleChoice(data) => edit_options(ui, &mut data.options, "Option"),
            Self::Checkboxes(data) => edit_options(ui, &mut data.options, "Option"),
            Self::Dropdown(data) => edit_options(ui, &mut data.options, "Option"),
            Self::LinearScale(data) => {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut data.start).clamp_range(0..=1));
                    ui.label("to");
                    ui.add(egui::DragValue::new(&mut data.end).clamp_range(2..=10));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}", data.start));
                    ui.add(
                        egui::TextEdit::singleline(&mut data.start_label)
                            .hint_text("Label (optional)")
                            .desired_width(100.0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}", data.end));
                    ui.add(
                        egui::TextEdit::singleline(&mut data.end_label)
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

    pub fn preview(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::ShortAnswer(data) => {
                ui.add(egui::TextEdit::singleline(&mut data.text).hint_text("Your answer"));
            }
            Self::Paragraph(data) => {
                ui.add(egui::TextEdit::multiline(&mut data.text).hint_text("Your answer"));
            }
            Self::MultipleChoice(data) => {
                for option in data.options.iter() {
                    ui.radio_value(&mut data.choice, option.clone(), option);
                }
            }
            Self::Checkboxes(data) => {
                for (i, option) in data.options.iter().enumerate() {
                    ui.checkbox(&mut data.choices[i], option);
                }
            }
            Self::Dropdown(data) => {
                let label = if data.choice.is_empty() {
                    "Choose".into()
                } else {
                    data.choice.clone()
                };
                ui.menu_button(label, |ui| {
                    for option in data.options.iter() {
                        if ui
                            .selectable_value(&mut data.choice, option.clone(), option)
                            .clicked()
                        {
                            ui.close_menu();
                        }
                    }
                });
            }
            Self::LinearScale(data) => {
                ui.horizontal(|ui| {
                    for i in data.start..=data.end {
                        ui.radio_value(&mut data.value, i, i.to_string());
                    }
                });
            }
            Self::MultipleChoiceGrid(data) => {
                egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    ui.label("");
                    for column in data.columns.iter() {
                        ui.label(column);
                    }
                    ui.end_row();
                    for (y, row) in data.rows.iter().enumerate() {
                        ui.label(row);
                        for column in data.columns.iter() {
                            ui.radio_value(&mut data.choices[y], column.clone(), "");
                        }
                        ui.end_row();
                    }
                });
            }
            Self::CheckboxGrid(data) => {
                egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                    ui.label("");
                    for column in data.columns.iter() {
                        ui.label(column);
                    }
                    ui.end_row();
                    for (y, row) in data.rows.iter().enumerate() {
                        ui.label(row);
                        for x in 0..data.columns.len() {
                            ui.checkbox(&mut data.choices[y][x], "");
                        }
                        ui.end_row();
                    }
                });
            }
            Self::Date(data) => {
                ui.label("MM  DD  YYYY");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut data.month).clamp_range(1..=12));
                    ui.label("/");
                    ui.add(egui::DragValue::new(&mut data.day).clamp_range(1..=31));
                    ui.label("/");
                    ui.add(egui::DragValue::new(&mut data.year).clamp_range(0..=9999));
                });
            }
            Self::Time(data) => {
                ui.label("Time");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut data.hour).clamp_range(1..=12));
                    ui.label(":");
                    ui.add(egui::DragValue::new(&mut data.minute).clamp_range(0..=59));
                    ui.menu_button(
                        match data.period {
                            types::DayPeriod::AM => "AM",
                            types::DayPeriod::PM => "PM",
                        },
                        |ui| {
                            if ui
                                .selectable_value(&mut data.period, types::DayPeriod::AM, "AM")
                                .clicked()
                            {
                                ui.close_menu();
                            }
                            if ui
                                .selectable_value(&mut data.period, types::DayPeriod::PM, "PM")
                                .clicked()
                            {
                                ui.close_menu();
                            }
                        },
                    );
                });
            }
        }
    }
}

impl PartialEq for QuestionData {
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

fn edit_grid(ui: &mut egui::Ui, rows: &mut Vec<String>, columns: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| edit_options(ui, rows, "Row"));
        ui.vertical(|ui| edit_options(ui, columns, "Column"));
    });
}

fn edit_options(ui: &mut egui::Ui, options: &mut Vec<String>, label: impl Into<String>) {
    let label: String = label.into();
    ui.label(format!("{}s", label));
    let mut delete_option = None;
    for (i, option) in options.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(option).desired_width(100.0));
            if ui.button("❌").clicked() {
                delete_option = Some(i);
            }
        });
    }
    if let Some(i) = delete_option {
        options.remove(i);
    }
    if ui.button(format!("Add {}", label.to_lowercase())).clicked() {
        options.push(format!("{} {}", label, options.len() + 1));
    }
}
