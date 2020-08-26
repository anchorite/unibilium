use crate::Term;
use std::ffi::CStr;
use std::fmt;
use unibilium_sys::unibi_boolean;

pub struct Boolean<'a> {
    boolean: unibi_boolean,
    term: &'a Term,
}

impl<'a> Boolean<'a> {
    pub(crate) fn from_unibi_bool(boolean: unibi_boolean, term: &'a Term) -> Self {
        Boolean { boolean, term }
    }

    pub fn name(&self) -> &str {
        // Returns static string if called with value between begin and end.
        let name = unsafe { unibilium_sys::unibi_name_bool(self.boolean) };
        if name.is_null() {
            panic!("Invalid unibi_bool value: {}", self.boolean);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    pub fn supported(&self) -> bool {
        let result = unsafe { unibilium_sys::unibi_get_bool(self.term.unibi_term(), self.boolean) };
        result > 0
    }
}

impl<'a> fmt::Display for Boolean<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.supported())
    }
}

pub struct ExtBoolean<'a> {
    index: u64,
    term: &'a Term,
}

impl<'a> ExtBoolean<'a> {
    pub(crate) fn from_index(index: u64, term: &'a Term) -> Self {
        ExtBoolean { index, term }
    }

    pub fn name(&self) -> &str {
        // Returns static string if called with value between 0 and count
        let name =
            unsafe { unibilium_sys::unibi_get_ext_bool_name(self.term.unibi_term(), self.index) };
        if name.is_null() {
            panic!("Invalid index for extended bool capability: {}", self.index);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    pub fn supported(&self) -> bool {
        let result =
            unsafe { unibilium_sys::unibi_get_ext_bool(self.term.unibi_term(), self.index) };
        result > 0
    }
}

impl<'a> fmt::Display for ExtBoolean<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.supported())
    }
}
