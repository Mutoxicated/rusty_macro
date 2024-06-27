use std::{path::PathBuf, slice::Iter, str::FromStr};

use eframe::App;
use egui::{Color32, Id};

use crate::{construct, csmacro::definition::MacroDefinition, csmacro::call::MacroCall, csstruct, helpers};

use super::app::{self, executer::{Executer, ExecutionError}, state::State};

csstruct!(
    Dresser;
    app:app::App,
    name_buf:String,
    parameters_buf:Vec<String>,
    pub code_buf:String,
    current_selected_macro:String,
    macro_arg_err:Option<String>,
    exec_err:Option<ExecutionError>
);

construct!(
    Dresser;
    app = app::App::new(),
    name_buf = String::new(),
    parameters_buf = Vec::new(),
    code_buf = String::from("Lorem Ipsum n shit"),
    current_selected_macro = String::from("Macro"),
    macro_arg_err = None,
    exec_err = None;
    value {
        let state = &mut value.app.states[value.app.current];
        Dresser::update_selected_macro(&mut value.current_selected_macro ,state.macro_def_iter());
    }
);

impl Dresser {
    pub fn update_selected_macro(current:&mut String, mut iter: Iter<'_, MacroDefinition>) {
        if iter.len() == 0 {
            *current = "Macro".to_owned();
            return;
        }
        if current != "Macro" {
            for def in iter.as_ref() {
                if def.name() == current {
                    return;
                }
            }
        }

       *current = iter.nth(0).unwrap().name().to_owned();
    }
}

impl App for Dresser {
    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        self.app.save_data();
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let states_length = self.app.states.len();
        egui::SidePanel::new(egui::panel::Side::Left, Id::new("Settings")).show(ctx, |ui| {
            ui.set_min_width(400.0);
            // SETTINGS
            ui.horizontal(|h| {
                let but = h.button("+");
                if but.clicked() {
                    self.app.states.push(State::new());
                }
                egui::ComboBox::from_label("Select a state").selected_text(self.app.current.to_string()).show_ui(h, |cui| {
                    for i in 0..states_length {
                        let but = cui.button(i.to_string());
                        if but.clicked() {
                            self.app.current = i;
                            break;
                        }
                    }
                });
            });

            ui.separator();

            let state = &mut self.app.states[self.app.current];

            let mut valid_macro = true;

            ui.label("Choose a path to the file");

            let but = ui.button("Open");
            if but.clicked() {

                let selected = tinyfiledialogs::open_file_dialog("Rusty Locounter", &self.app.current_path, None);
                if let Some(str) = selected {
                    state.set_spath(str.as_str());
                    app::App::set_current_path(&mut self.app.current_path, str.as_str());
                }
            }

            if let Some(x) = state.spath() {
                ui.horizontal(|h| {
                    h.label(x);
                    if state.spath().is_some() { 
                        h.menu_button("Info", |mui| {
                            mui.colored_label(Color32::GREEN, "For this to work you have to write the macro start (\"// MACRO START\") and end (\"// MACRO END\") flag in the script.");
                        });
                    }
                });
            }
            
            ui.menu_button("Add macro definition", |mui| {
                mui.text_edit_singleline(&mut self.name_buf);
                if self.name_buf.is_empty() {
                    mui.colored_label(Color32::RED, "Macro name is empty!");
                    valid_macro = false;
                }else if state.contains_macro_def(self.name_buf.as_str()) {
                    mui.colored_label(Color32::RED, "There is already a macro with that name!");
                    valid_macro = false;
                }
                mui.label("Parameters");
                let param = mui.button("New");
                if param.clicked() {
                    self.parameters_buf.push("".to_owned())
                }
                let buflen = self.parameters_buf.len();
                for i in 0..buflen {
                    mui.text_edit_singleline(&mut self.parameters_buf[i]);
                }
                
                if helpers::identical_elems(&self.parameters_buf) {
                    mui.colored_label(Color32::RED, "Some parameters have the same name!");
                    valid_macro = false
                }else if helpers::contains(&self.parameters_buf, String::new()).is_ok() {
                    mui.colored_label(Color32::RED, "One or more parameters is empty!");
                    valid_macro = false
                }else if self.parameters_buf.is_empty() {
                    mui.colored_label(Color32::RED, "There needs to be at least 1 parameter!");
                    valid_macro = false
                }

                mui.label("Code");
                mui.code_editor(&mut self.code_buf);

                let but = mui.button("Add");
                if but.clicked() && valid_macro {
                    state.add_macro_def(
                        MacroDefinition::new(&self.name_buf)
                            .set_params(self.parameters_buf.clone())
                            .set_code(self.code_buf.clone())
                    );
                    Dresser::update_selected_macro(&mut self.current_selected_macro, state.macro_def_iter());
                    self.code_buf = String::from("Lorem Ipsum n shit");
                    self.parameters_buf.clear();
                    self.name_buf = String::new();
                }
            });

            // MACRO DEFINITIONS
            if state.macro_def_len() != 0 {
                ui.colored_label(Color32::GREEN, "--Macro definitions");
                egui::ScrollArea::vertical().show_rows(ui, 0.0, state.macro_def_len(), |sui, range| {
                    for i in range {
                        let macdef = state.macro_def_mut(i);
                        sui.colored_label(Color32::DARK_GREEN,"-".to_owned()+macdef.name());
                        sui.label("Params:");
                        let iter = macdef.params_iter_mut();
                        for param in iter {
                            sui.text_edit_singleline(param);
                        }
    
                        sui.label("Code:");
                        sui.code_editor(macdef.code_string_mut());
                        let remove = sui.button("Remove");
                        if remove.clicked() {
                            state.remove_macro_def(i);
                            Dresser::update_selected_macro(&mut self.current_selected_macro, state.macro_def_iter());
                            break;
                        }
                    }
                });
            }
        });

        egui::SidePanel::right("Execution Setup").show(ctx, |ui| {
            let state = &mut self.app.states[self.app.current];

            ui.set_min_width(600.0);
            egui::ComboBox::from_label("Select a macro").selected_text(&self.current_selected_macro).show_ui(ui, |cui| {
                for def in state.macro_def_iter() {
                    let but = cui.button(def.name());
                    if but.clicked() {
                        self.current_selected_macro = def.name().to_owned();
                    }
                }
            });
            let add = ui.button("Add");
           
            if add.clicked() {
                if state.macro_def_len() == 0 {
                    self.macro_arg_err = Some("There are no macro definitions!".to_owned());
                    return;
                }
                let call = MacroCall::new(
                    state.macro_def_iter().find(|x| {x.name() == self.current_selected_macro}).unwrap().clone()
                );

                state.add_macro_call(call);
            }

            // CALL LIST
            ui.colored_label(Color32::GREEN, "--Call List");
            
            egui::ScrollArea::vertical().show_rows(ui, 0.0, state.macro_call_len(), |sui, _| {
                for call in state.macro_call_iter_mut() {
                    let def = call.definition();
                    sui.colored_label(Color32::DARK_GREEN, "-".to_owned()+def.name());
                    for arg in call.arg_iter_mut() {
                        sui.horizontal(|h| {
                            h.label(arg.0);
                            h.text_edit_singleline(arg.1);
                        });
                    }
                }
            });

            let action = ui.button("Action");
            if action.clicked() && state.spath().is_some() {
                let mut executer = Executer::new(PathBuf::from_str(state.spath().as_ref().unwrap().as_str()).unwrap());
                
                let res = executer.action(state.macro_calls());
                if let Err(x) = res {
                    self.exec_err = Some(x);
                }
            }else if action.clicked() {
                self.exec_err = Some(ExecutionError::NoFile);
            }
            if let Some(x) = &self.exec_err {
                ui.colored_label(Color32::RED, format!("{}", x));
            }
        });
    }
}