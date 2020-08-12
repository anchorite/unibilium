use std::ffi::CStr;
use std::fmt;
use unibilium_sys::{
    unibi_boolean, unibi_from_env, unibi_from_term, unibi_numeric, unibi_string, unibi_term,
};

pub struct Term {
    term: *mut unibi_term,
}

pub struct Boolean<'a> {
    boolean: unibi_boolean,
    term: &'a Term,
}

impl<'a> Boolean<'a> {
    fn from_unibi_bool(boolean: unibi_boolean, term: &'a Term) -> Self {
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
        let result = unsafe { unibilium_sys::unibi_get_bool(self.term.term, self.boolean) };
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
    fn from_index(index: u64, term: &'a Term) -> Self {
        ExtBoolean { index, term }
    }

    pub fn name(&self) -> &str {
        // Returns static string if called with value between 0 and count
        let name = unsafe { unibilium_sys::unibi_get_ext_bool_name(self.term.term, self.index) };
        if name.is_null() {
            panic!("Invalid index for extended bool capability: {}", self.index);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    pub fn supported(&self) -> bool {
        let result = unsafe { unibilium_sys::unibi_get_ext_bool(self.term.term, self.index) };
        result > 0
    }
}

impl<'a> fmt::Display for ExtBoolean<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.supported())
    }
}

pub struct Numeric<'a> {
    term: &'a Term,
    numeric: unibi_numeric,
}

impl<'a> Numeric<'a> {
    fn from_unibi_numeric(numeric: unibi_numeric, term: &'a Term) -> Self {
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
        unsafe { unibilium_sys::unibi_get_num(self.term.term, self.numeric) }
    }
}

impl<'a> fmt::Display for Numeric<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name(), self.value())
    }
}

pub struct String<'a> {
    term: &'a Term,
    string: unibi_string,
}

impl<'a> String<'a> {
    fn from_unibi_string(string: unibi_string, term: &'a Term) -> Self {
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
        let value = unsafe { unibilium_sys::unibi_get_str(self.term.term, self.string) };
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
        self.value().map(|value| {
            std::string::String::from_utf8(
                value
                    .as_bytes()
                    .iter()
                    .map(|c| std::ascii::escape_default(*c))
                    .flatten()
                    .collect(),
            )
            .expect("Invalid UTF-8 string encountered")
            .replace("\\x1b", "^[")
        })
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

impl Term {
    pub fn from_env() -> Term {
        Term {
            term: unsafe { unibi_from_env() },
        }
    }

    pub fn from_term_name(name: &str) -> Term {
        Term {
            term: unsafe { unibi_from_term(name.as_ptr() as *const i8) },
        }
    }

    pub fn booleans(&self) -> Vec<Boolean> {
        let mut all = vec![];
        let first = unibi_boolean::unibi_boolean_begin_.0 + 1;
        let end = unibi_boolean::unibi_boolean_end_.0;
        for current in first..end {
            let b = Boolean::from_unibi_bool(unibi_boolean(current), self);
            all.push(b);
        }
        all
    }

    pub fn ext_booleans(&self) -> Vec<ExtBoolean> {
        let mut all = vec![];
        let end = unsafe { unibilium_sys::unibi_count_ext_bool(self.term) };
        for index in 0..end {
            let b = ExtBoolean::from_index(index, self);
            all.push(b);
        }
        all
    }

    pub fn numerics(&self) -> Vec<Numeric> {
        let mut all = vec![];
        let first = unibi_numeric::unibi_numeric_begin_.0 + 1;
        let end = unibi_numeric::unibi_numeric_end_.0;
        for current in first..end {
            let n = Numeric::from_unibi_numeric(unibi_numeric(current), self);
            all.push(n);
        }
        all
    }

    pub fn strings(&self) -> Vec<String> {
        let mut all = vec![];
        let first = unibi_string::unibi_string_begin_.0 + 1;
        let end = unibi_string::unibi_string_end_.0;
        for current in first..end {
            let s = String::from_unibi_string(unibi_string(current), self);
            all.push(s);
        }
        all
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        unsafe {
            unibilium_sys::unibi_destroy(self.term);
        }
    }
}
