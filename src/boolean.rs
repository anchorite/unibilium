use crate::Term;
use std::ffi::CStr;
use std::fmt;
use unibilium_sys::unibi_boolean;

/// Represents boolean terminal capability.
#[derive(Debug)]
pub struct Boolean<'a> {
    boolean: unibi_boolean,
    term: &'a Term,
}

impl<'a> Boolean<'a> {
    /// Creates a Boolean from lower level unibi_boolean struct. It's intended for internal
    /// use.
    ///
    /// # Warning
    ///
    /// For efficiency reasons it expects the caller to make sure that the boolean is valid.
    pub(crate) fn from_unibi_bool_unchecked(boolean: unibi_boolean, term: &'a Term) -> Self {
        Boolean { boolean, term }
    }

    /// Returns name of the capability.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::term::Term;
    ///
    /// let term = Term::from_env()?;
    /// let bool_caps = term.booleans();
    /// println!("A boolean capability name: {}", bool_caps.first().unwrap().name());
    /// #
    /// #    Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// If it encounters invalid UTF-8 characters it will panic.
    pub fn name(&self) -> &str {
        // Returns static string if called with value between begin and end.
        let name = unsafe { unibilium_sys::unibi_name_bool(self.boolean) };
        if name.is_null() {
            panic!("Invalid unibi_bool value: {}", self.boolean);
        }
        let name = unsafe { CStr::from_ptr(name) };
        name.to_str().expect("Invalid UTF-8 string encountered")
    }

    /// Returns whether the associated terminal supports this capability.
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

/// Represents extended boolean capabilities. Each terminal can support custom boolean
/// capabilities.
#[derive(Debug)]
pub struct ExtBoolean<'a> {
    index: u64,
    term: &'a Term,
}

impl<'a> ExtBoolean<'a> {
    /// Creates an ExtBoolean from an index.
    ///
    /// # Warning
    ///
    /// For efficiency reasons it expects the caller to make sure that the index is valid, i.e.
    /// less than unibilium_sys::unibi_count_ext_bool.
    pub(crate) fn from_index_unchecked(index: u64, term: &'a Term) -> Self {
        ExtBoolean { index, term }
    }

    /// Returns the name of the extended boolean capability.
    ///
    /// # Panics
    ///
    /// If it encounters an invalid UTF-8 character.
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

    /// Returns whether the associated terminal supports this capability.
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
