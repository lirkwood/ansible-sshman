use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct UndefinedSectionError {
    pub name: String
}
impl Error for UndefinedSectionError {}
impl Display for UndefinedSectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Undefined section: {}", self.name)
    }
}

#[derive(Debug)]
pub struct InvalidConfigError {
    pub message: String
}
impl Error for InvalidConfigError {}
impl Display for InvalidConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid SSH config file; {}", self.message)
    }
}
impl InvalidConfigError {
    pub fn from_str(str: &str) -> Self {
        return InvalidConfigError { message: str.to_string() }
    }
}