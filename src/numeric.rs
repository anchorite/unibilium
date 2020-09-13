use crate::term::Term;
use std::ffi::CStr;
use std::fmt;
use unibilium_sys::unibi_numeric;

/// Represents numeric terminal capability.
#[derive(Debug)]
pub struct Numeric<'a> {
    term: &'a Term,
    numeric: unibi_numeric,
}

impl<'a> Numeric<'a> {
    /// Creates a Numeric from lower level unibi_numeric struct. It's intended for internal
    /// use.
    ///
    /// # Warning
    ///
    /// For efficiency reasons it expects the caller to make sure that the numeric parameter is
    /// valid.
    pub(crate) fn from_unibi_numeric_unchecked(numeric: unibi_numeric, term: &'a Term) -> Self {
        Numeric { numeric, term }
    }

    /// Returns name of the capability.
    ///
    /// # Panics
    ///
    /// If it encounters invalid UTF-8 characters it will panic.
    pub fn name(&self) -> &str {
        // Returns static string if called with value between begin and end.
        let name = unsafe { unibilium_sys::unibi_name_num(self.numeric) };
        if name.is_null() {
            panic!("Invalid unibi_numeric value: {}", self.numeric);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    /// Returns the value corresponding to the numeric terminal capability.
    pub fn value(&self) -> i32 {
        unsafe { unibilium_sys::unibi_get_num(self.term.unibi_term(), self.numeric) }
    }
}

impl<'a> fmt::Display for Numeric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.value())
    }
}

/// Represents extended numeric capabilities. Each terminal can support custom numeric
/// capabilities.
#[derive(Debug)]
pub struct ExtNumeric<'a> {
    index: u64,
    term: &'a Term,
}

impl<'a> ExtNumeric<'a> {
    /// Creates an ExtBoolean from an index.
    ///
    /// # Warning
    ///
    /// For efficiency reasons it expects the caller to make sure that the index is valid, i.e.
    /// less than unibilium_sys::unibi_count_ext_num.
    pub(crate) fn from_index_unchecked(index: u64, term: &'a Term) -> Self {
        ExtNumeric { index, term }
    }

    /// Returns the name of the extended numeric capability.
    ///
    /// # Panics
    ///
    /// If it encounters an invalid UTF-8 character.
    pub fn name(&self) -> &str {
        // Returns static string if called with value between 0 and count
        let name =
            unsafe { unibilium_sys::unibi_get_ext_num_name(self.term.unibi_term(), self.index) };
        if name.is_null() {
            panic!(
                "Invalid index for extended numeric capability: {}",
                self.index
            );
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    /// Returns the value corresponding to the extended numeric terminal capability.
    pub fn value(&self) -> i32 {
        unsafe { unibilium_sys::unibi_get_ext_num(self.term.unibi_term(), self.index) }
    }
}

impl<'a> fmt::Display for ExtNumeric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.value())
    }
}
