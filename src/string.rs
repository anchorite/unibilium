use crate::term::Term;
use std::ffi::CStr;
use std::fmt;
use unibilium_sys::unibi_string;

#[derive(Debug)]
pub struct String<'a> {
    term: &'a Term,
    string: unibi_string,
}

impl<'a> String<'a> {
    pub(crate) fn from_unibi_string_unchecked(string: unibi_string, term: &'a Term) -> Self {
        String { string, term }
    }

    pub fn name(&self) -> &str {
        // Returns static string if called with value between begin and end.
        let name = unsafe { unibilium_sys::unibi_name_str(self.string) };
        if name.is_null() {
            panic!("Invalid unibi_string value: {}", self.string);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    pub fn value(&self) -> Option<&str> {
        let value = unsafe { unibilium_sys::unibi_get_str(self.term.unibi_term(), self.string) };
        if value.is_null() {
            return None;
        }
        let value = unsafe { CStr::from_ptr(value) };
        Some(value.to_str().expect("Invalid UTF-8 string encountered"))
    }

    /// Returns escaped std::string::String representing the value of the capability. Escaping is
    /// done according to the rules by std::ascii::escape_default, with the exception that
    /// escape(0x1b) is represented as '^['.
    pub fn escaped_value(&self) -> Option<std::string::String> {
        escape_string(self.value())
    }
}

impl<'a> fmt::Display for String<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.escaped_value() {
            None => write!(f, "{}: NULL", self.name()),
            Some(value) => write!(f, "{}: {}", self.name(), value),
        }
    }
}

/// Represents extended string terminal capabilities.
#[derive(Debug)]
pub struct ExtString<'a> {
    index: u64,
    term: &'a Term,
}

impl<'a> ExtString<'a> {
    pub(crate) fn from_index_unchecked(index: u64, term: &'a Term) -> Self {
        ExtString { index, term }
    }

    pub fn name(&self) -> &str {
        // Returns static string if called with value between 0 and count
        let name =
            unsafe { unibilium_sys::unibi_get_ext_str_name(self.term.unibi_term(), self.index) };
        if name.is_null() {
            panic!(
                "Invalid index for extended string capability: {}",
                self.index
            );
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    pub fn value(&self) -> Option<&str> {
        let value = unsafe { unibilium_sys::unibi_get_ext_str(self.term.unibi_term(), self.index) };
        if value.is_null() {
            return None;
        }
        let value = unsafe { CStr::from_ptr(value) };
        Some(value.to_str().expect("Invalid UTF-8 string encountered"))
    }

    /// Returns escaped std::string::String representing the value of the capability. Escaping is
    /// done according to the rules by std::ascii::escape_default, with the exception that
    /// escape(0x1b) is represented as '^['.
    pub fn escaped_value(&self) -> Option<std::string::String> {
        escape_string(self.value())
    }
}

impl<'a> fmt::Display for ExtString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.escaped_value() {
            None => write!(f, "{}: NULL", self.name()),
            Some(value) => write!(f, "{}: {}", self.name(), value),
        }
    }
}

fn escape_string(s: Option<&str>) -> Option<std::string::String> {
    s.map(|s| {
        std::string::String::from_utf8(
            s.as_bytes()
                .iter()
                .map(|c| std::ascii::escape_default(*c))
                .flatten()
                .collect(),
        )
        .expect("Invalid UTF-8 string encountered")
        .replace("\\x1b", "^[")
    })
}
