use std::collections::HashMap;

use crate::CLIError;

#[derive(Debug, Clone)]
pub struct OptionArg {
    pub short: Option<String>,
    pub long: Option<String>,
    pub description: String,
    pub takes_value: bool,
    pub default_value: Option<String>,
    pub required: bool,
}

impl OptionArg {
    pub fn new(short: Option<&str>, long: Option<&str>, description: &str) -> Self {
        Self {
            short: short.map(|s| s.to_string()),
            long: long.map(|l| l.to_string()),
            description: description.to_string(),
            takes_value: false,
            default_value: None,
            required: false,
        }
    }

    pub fn takes_value(mut self, value: bool) -> Self {
        self.takes_value = value;
        self
    }

    pub fn default_value(mut self, value: &str) -> Self {
        self.default_value = Some(value.to_string());
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn validate(&self, options: &HashMap<String, String>) -> Result<(), CLIError> {
        if self.required && !options.contains_key(self.long.as_ref().unwrap()) {
            return Err(CLIError::MissingOption(self.long.clone().unwrap()));
        }
        Ok(())
    }
}
