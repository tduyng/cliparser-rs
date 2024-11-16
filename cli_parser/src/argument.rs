use std::fmt;

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub required: bool,
}

impl Argument {
    pub fn new(name: &str, required: bool) -> Self {
        Self {
            name: name.to_string(),
            required,
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.required {
            write!(f, "<{}>", self.name)
        } else {
            write!(f, "[{}]", self.name)
        }
    }
}
