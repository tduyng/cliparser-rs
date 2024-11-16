use crate::{option::OptionArg, Argument, CLIError};
use std::collections::HashMap;

type CommandAction = Box<dyn Fn(Vec<String>, HashMap<String, String>)>;

pub struct Command {
    pub name: String,
    pub description: String,
    pub arguments: Vec<Argument>,
    pub options: Vec<OptionArg>,
    pub subcommands: HashMap<String, Command>,
    pub action: Option<CommandAction>,
    pub alias: Option<String>,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            arguments: Vec::new(),
            options: Vec::new(),
            subcommands: HashMap::new(),
            action: None,
            alias: None,
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn argument(mut self, arg: Argument) -> Self {
        self.arguments.push(arg);
        self
    }

    pub fn option(mut self, opt: OptionArg) -> Self {
        self.options.push(opt);
        self
    }

    pub fn subcommand(mut self, cmd: Command) -> Self {
        self.subcommands.insert(cmd.name.clone(), cmd);
        self
    }

    pub fn alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    pub fn action<F>(mut self, callback: F) -> Self
    where
        F: Fn(Vec<String>, HashMap<String, String>) + 'static,
    {
        self.action = Some(Box::new(callback));
        self
    }

    fn resolve_alias(&self, name: &str) -> Option<&Command> {
        if let Some(ref alias) = self.alias {
            if alias == name {
                return Some(self);
            }
        }
        None
    }

    pub fn parse(&self, args: &[String]) -> Result<(), CLIError> {
        let (name, remaining_args) = args.split_first().ok_or(CLIError::MissingCommand)?;

        if name == "--help" || name == "-h" {
            self.display_help();
            return Ok(());
        }

        if let Some(command) = self.resolve_alias(name) {
            return command.parse(remaining_args);
        }

        if let Some(subcommand) = self.subcommands.get(name) {
            return subcommand.parse(remaining_args);
        }

        let mut parsed_options = HashMap::new();
        let mut parsed_args = Vec::new();

        for arg in args {
            if arg.starts_with("--") {
                let (key, value) = arg
                    .split_once('=')
                    .unwrap_or((arg.trim_start_matches("--"), ""));
                parsed_options.insert(key.trim_start_matches("--").to_string(), value.to_string());
            } else if arg.starts_with('-') {
                let (key, value) = arg
                    .split_once('=')
                    .unwrap_or((arg.trim_start_matches("-"), ""));
                parsed_options.insert(key.trim_start_matches("-").to_string(), value.to_string());
            } else {
                parsed_args.push(arg.clone());
            }
        }

        for argument in &self.arguments {
            if argument.required && !parsed_args.contains(&argument.name) {
                return Err(CLIError::MissingArgument(argument.name.clone()));
            }
        }

        if let Some(action) = &self.action {
            action(parsed_args, parsed_options);
        } else {
            return Err(CLIError::MissingAction);
        }

        Ok(())
    }

    fn display_help(&self) {
        let mut help_text = format!("Usage: {}\n\n", self.name);
        help_text.push_str(&format!("Description: {}\n\n", self.description));

        if !self.arguments.is_empty() {
            help_text.push_str("Arguments:\n");
            for arg in &self.arguments {
                help_text.push_str(&format!("  {}\n", arg));
            }
        }

        if !self.options.is_empty() {
            help_text.push_str("Options:\n");
            for opt in &self.options {
                let flags = match (&opt.short, &opt.long) {
                    (Some(s), Some(l)) => format!("-{}, --{}", s, l),
                    (Some(s), None) => format!("-{}", s),
                    (None, Some(l)) => format!("--{}", l),
                    _ => String::new(),
                };
                help_text.push_str(&format!("  {:<20} {}\n", flags, opt.description));
            }
        }

        if !self.subcommands.is_empty() {
            help_text.push_str("Subcommands:\n");
            for sub in self.subcommands.keys() {
                help_text.push_str(&format!("  {}\n", sub));
            }
        }

        println!("{}", help_text);
    }
}
