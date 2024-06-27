use std::{slice::{Iter, IterMut}};

use serde::{Deserialize, Serialize};

use crate::{construct, csmacro::{call::MacroCall, definition::MacroDefinition}};


#[derive(Deserialize, Serialize)]
pub struct State {
    script_path:Option<String>,

    macro_definitions:Vec<MacroDefinition>,
    macro_calls:Vec<MacroCall>,
}

construct!(
    State;
    script_path = None,

    macro_definitions = Vec::new(),
    macro_calls = Vec::new()
);

impl State {
    
    pub fn set_spath(&mut self, path:&str) {
        self.script_path = Some(path.to_owned())
    }

    pub fn spath(&self) -> &Option<String> {
        &self.script_path
    }

    pub fn contains_macro_def(&self, name:&str) -> bool {
        for mcdef in &self.macro_definitions {
            if mcdef.name() == name {
                return true
            }
        }
        false
    }

    // Macro Definitions
    pub fn add_macro_def(&mut self, macro_def:MacroDefinition) {
        self.macro_definitions.push(macro_def);
    }

    pub fn remove_macro_def(&mut self, i:usize) {
        for j in (0..self.macro_call_len()).rev() {
            if self.macro_calls[j].definition() == &self.macro_definitions[i] {
                self.macro_calls.remove(j);
            }
        }
        self.macro_definitions.remove(i);
    }

    pub fn macro_def_mut(&mut self, i:usize) -> &mut MacroDefinition{
        &mut self.macro_definitions[i]
    }

    pub fn macro_def_iter_mut(&mut self) -> IterMut<'_, MacroDefinition> {
        self.macro_definitions.iter_mut()   
    }

    pub fn macro_def_iter(&self) -> Iter<'_, MacroDefinition> {
        self.macro_definitions.iter()   
    }

    pub fn macro_def_len(&self) -> usize {
        self.macro_definitions.len()
    }

    // Macro Calls
    pub fn add_macro_call(&mut self, macro_call:MacroCall) {
        self.macro_calls.push(macro_call);
    }

    pub fn remove_macro_call(&mut self, i:usize) {
        self.macro_calls.remove(i);
    }

    pub fn macro_call_iter_mut(&mut self) -> IterMut<'_, MacroCall> {
        self.macro_calls.iter_mut()
    }
    pub fn macro_call_iter(&self) -> Iter<'_, MacroCall> {
        self.macro_calls.iter()
    }

    pub fn macro_call_len(&self) -> usize {
        self.macro_calls.len()
    }

    pub fn macro_calls(&self) -> &Vec<MacroCall> {
        &self.macro_calls
    }
}