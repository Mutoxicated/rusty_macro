use std::slice::{Iter, IterMut};

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
pub struct MacroDefinition {
    name: String,
    parameters: Vec<String>,
    code: String
}

impl PartialEq for MacroDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for MacroDefinition {
    fn assert_receiver_is_total_eq(&self) {
        
    }
}

impl MacroDefinition {
    pub fn new(name:&str) -> Self {
        Self {
            name:name.to_owned(),
            parameters: Vec::new(),
            code: String::new()
        }
    }

    pub fn set_code(mut self, code: String) -> Self {
        self.code = code;
        self
    }

    pub fn set_params(mut self, params:Vec<String>) -> Self {
        self.parameters = params;
        self
    }

    pub fn params_iter(&self) -> Iter<'_, String> {
        self.parameters.iter()
    } 

    pub fn params_iter_mut(&mut self) -> IterMut<'_, String> {
        self.parameters.iter_mut()
    } 

    pub fn new_parameter(&mut self) {
        self.parameters.push(String::new());
    }

    pub fn parameter_mut(&mut self, i: usize) -> &mut str {
        self.parameters[i].as_mut_str()
    }

    pub fn parameter(&self, i: usize) -> &str {
        self.parameters[i].as_str()
    }

    pub fn parameters_len(&self) -> usize {
        self.parameters.len()
    }

    pub fn code_mut(&mut self) -> &mut str {
        &mut self.code
    }

    pub fn code_string_mut(&mut self) -> &mut String {
        &mut self.code
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}