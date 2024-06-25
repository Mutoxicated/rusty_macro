use std::{borrow::Borrow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::definition::CSMacroDefinition;

#[derive(Deserialize, Serialize)]
pub struct CSMacroCall {
    definition: CSMacroDefinition,
    arguments:HashMap<String, String>
}

impl CSMacroCall {
    pub fn new(definition:CSMacroDefinition) -> Self {
        let mut arguments:HashMap<String, String> = HashMap::new();

        for param in definition.params_iter() {
            arguments.insert(param.clone(), "".to_owned());
        }

        Self {
            definition,
            arguments
        }
    }

    pub fn expand(&self) -> String {
        let code = self.definition.code().to_owned();
        for (k, v) in self.arguments.borrow() {
            let _ = code.replace((String::from("$")+k.as_str()).as_str(), v.as_str());
        }
        code
    }
}