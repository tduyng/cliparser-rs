#[derive(Debug)]
pub enum CLIError {
    MissingCommand,
    MissingArgument(String),
    MissingOption(String),
    MissingAction,
    InvalidOption(String),
    InvalidSubcommand(String),
    InvalidFlag(String),
}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CLIError::MissingCommand => write!(f, "Missing command"),
            CLIError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            CLIError::MissingOption(opt) => write!(f, "Missing required option: {}", opt),
            CLIError::MissingAction => write!(f, "Missing required action"),
            CLIError::InvalidOption(opt) => write!(f, "Invalid option: {}", opt),
            CLIError::InvalidSubcommand(cmd) => write!(f, "Invalid subcommand: {}", cmd),
            CLIError::InvalidFlag(flag) => write!(f, "Invalid flag: {}", flag),
        }
    }
}

impl std::error::Error for CLIError {}
