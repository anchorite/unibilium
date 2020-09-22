use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TermError {
    NotFound(String),
    NotUnicode,
}

impl TermError {
    pub(crate) fn from_name(name: &str) -> Self {
        TermError::NotFound(String::from(name))
    }

    pub(crate) fn from_term_var() -> Self {
        use std::env::{var, VarError};

        match var("TERM") {
            Ok(value) => TermError::from_name(&value),
            Err(err) => match err {
                VarError::NotPresent => TermError::from_name(""),
                VarError::NotUnicode(_) => TermError::NotUnicode,
            },
        }
    }
}

impl Error for TermError {}

impl Display for TermError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TermError::NotFound(ref s) => write!(f, "terminfo not found by name '{}'", s),
            TermError::NotUnicode => write!(f, "non unicode string encountered"),
        }
    }
}
