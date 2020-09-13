use crate::boolean::{Boolean, ExtBoolean};
use crate::numeric::{ExtNumeric, Numeric};
use crate::string::{ExtString, String};
use std::ffi::CString;
use unibilium_sys::{
    unibi_boolean, unibi_from_env, unibi_from_term, unibi_numeric, unibi_string, unibi_term,
};

/// The main structure provided by this library. Used to represent and manipulate capabilities of a
/// terminal.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Term {
    term: *mut unibi_term,
}

impl Term {
    /// Creates a Term struct from terminal according to TERM environment variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use unibilium::Term;
    ///
    /// let term = Term::from_env().expect("Fails to create from TERM variable");
    /// ```
    ///
    /// TODO: return Result with crate specific Error
    pub fn from_env() -> Option<Term> {
        let term = unsafe { unibi_from_env() };
        if term.is_null() {
            None
        } else {
            Some(Term { term })
        }
    }

    /// Creates a Term struct from specified terminal name.
    ///
    /// # Examples
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// ```
    ///
    /// # Errors
    ///
    /// It returns `None` if name contains a null character or there is no terminfo file
    /// corresponding to a terminal with provided name.
    pub fn from_term_name(name: &str) -> Option<Term> {
        let name = CString::new(name).ok()?;
        let term = unsafe { unibi_from_term(name.as_ptr()) };
        if term.is_null() {
            None
        } else {
            Some(Term { term })
        }
    }

    /// Returns all boolean capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// for b in term.booleans() {
    ///     println!("{}", b);
    /// }
    /// ```
    pub fn booleans(&self) -> Vec<Boolean> {
        let mut all = vec![];
        let first = unibi_boolean::unibi_boolean_begin_.0 + 1;
        let end = unibi_boolean::unibi_boolean_end_.0;
        for current in first..end {
            let b = Boolean::from_unibi_bool_unchecked(unibi_boolean(current), self);
            all.push(b);
        }
        all
    }

    /// Returns all extended capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// for b in term.ext_booleans() {
    ///     println!("{}", b);
    /// }
    /// ```
    pub fn ext_booleans(&self) -> Vec<ExtBoolean> {
        let mut all = vec![];
        let end = unsafe { unibilium_sys::unibi_count_ext_bool(self.term) };
        for index in 0..end {
            let b = ExtBoolean::from_index_unchecked(index, self);
            all.push(b);
        }
        all
    }

    /// Returns all numeric capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// for b in term.numerics() {
    ///     println!("{}", b);
    /// }
    /// ```
    pub fn numerics(&self) -> Vec<Numeric> {
        let mut all = vec![];
        let first = unibi_numeric::unibi_numeric_begin_.0 + 1;
        let end = unibi_numeric::unibi_numeric_end_.0;
        for current in first..end {
            let n = Numeric::from_unibi_numeric_unchecked(unibi_numeric(current), self);
            all.push(n);
        }
        all
    }

    /// Returns all extended numeric capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// for b in term.ext_numerics() {
    ///     println!("{}", b);
    /// }
    /// ```
    pub fn ext_numerics(&self) -> Vec<ExtNumeric> {
        let mut all = vec![];
        let end = unsafe { unibilium_sys::unibi_count_ext_num(self.term) };
        for index in 0..end {
            let b = ExtNumeric::from_index_unchecked(index, self);
            all.push(b);
        }
        all
    }

    /// Returns all string capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// for b in term.strings() {
    ///     println!("{}", b);
    /// }
    /// ```
    pub fn strings(&self) -> Vec<String> {
        let mut all = vec![];
        let first = unibi_string::unibi_string_begin_.0 + 1;
        let end = unibi_string::unibi_string_end_.0;
        for current in first..end {
            let s = String::from_unibi_string_unchecked(unibi_string(current), self);
            all.push(s);
        }
        all
    }

    /// Returns all extended string capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)
    ///     .unwrap_or_else(|| panic!("Failed to find terminfo for '{}'", term_name));
    /// for b in term.ext_strings() {
    ///     println!("{}", b);
    /// }
    /// ```
    pub fn ext_strings(&self) -> Vec<ExtString> {
        let mut all = vec![];
        let end = unsafe { unibilium_sys::unibi_count_ext_str(self.term) };
        for index in 0..end {
            let b = ExtString::from_index_unchecked(index, self);
            all.push(b);
        }
        all
    }

    /// Returns the wrapped pointer to the C library structure. It is intended for internal use
    /// where the lower level structure needs to be passed.
    pub(crate) fn unibi_term(&self) -> *mut unibi_term {
        self.term
    }
}

impl Drop for Term {
    /// Calls the destructor for the low level C structure. Prevents leaks.
    fn drop(&mut self) {
        unsafe {
            unibilium_sys::unibi_destroy(self.term);
        }
    }
}
