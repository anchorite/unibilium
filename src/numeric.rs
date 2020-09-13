use crate::term::Term;
use std::ffi::CStr;
use std::fmt;
use unibilium_sys::unibi_numeric;

#[derive(Debug)]
pub struct Numeric<'a> {
    term: &'a Term,
    numeric: unibi_numeric,
}

impl<'a> Numeric<'a> {
    pub(crate) fn from_unibi_numeric_unchecked(numeric: unibi_numeric, term: &'a Term) -> Self {
        Numeric { numeric, term }
    }

    pub fn name(&self) -> &str {
        // Returns static string if called with value between begin and end.
        let name = unsafe { unibilium_sys::unibi_name_num(self.numeric) };
        if name.is_null() {
            panic!("Invalid unibi_numeric value: {}", self.numeric);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    pub fn value(&self) -> i32 {
        unsafe { unibilium_sys::unibi_get_num(self.term.unibi_term(), self.numeric) }
    }
}

impl<'a> fmt::Display for Numeric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.value())
    }
}

#[derive(Debug)]
pub struct ExtNumeric<'a> {
    index: u64,
    term: &'a Term,
}

impl<'a> ExtNumeric<'a> {
    pub(crate) fn from_index_unchecked(index: u64, term: &'a Term) -> Self {
        ExtNumeric { index, term }
    }

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

    pub fn value(&self) -> i32 {
        unsafe { unibilium_sys::unibi_get_ext_num(self.term.unibi_term(), self.index) }
    }
}

impl<'a> fmt::Display for ExtNumeric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.value())
    }
}
