use crate::boolean::{Boolean, ExtBoolean};
use crate::error::TermError;
use crate::numeric::{ExtNumeric, Numeric};
use crate::string::{ExtString, String};
use std::error::Error;
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
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    ///
    /// let term = Term::from_env()?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// * TERM contains terminal without present termcap file
    /// * TERM contains non-UTF8 string
    pub fn from_env() -> Result<Term, Box<dyn Error>> {
        let term = unsafe { unibi_from_env() };
        if term.is_null() {
            Err(Box::new(TermError::from_term_var()))
        } else {
            Ok(Term { term })
        }
    }

    /// Creates a Term struct from specified terminal name.
    ///
    /// # Examples
    ///
    /// Succeeds for well known terminals.
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Returns error for unknown terminals.
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "blahblah2234";
    ///
    /// assert!(Term::from_term_name(term_name).is_err());
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// * name presents terminal without present termcap file
    /// * name presents non-UTF8 string
    pub fn from_term_name(name: &str) -> Result<Term, Box<dyn Error>> {
        let cname = match CString::new(name) {
            Ok(cname) => cname,
            Err(_) => return Err(Box::new(TermError::from_name(name))),
        };
        let term = unsafe { unibi_from_term(cname.as_ptr()) };
        if term.is_null() {
            Err(Box::new(TermError::from_name(name)))
        } else {
            Ok(Term { term })
        }
    }

    /// Returns all boolean capabilities for the terminal.
    ///
    /// # Examples
    ///
    /// Printing their values
    ///
    /// ```
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// for b in term.booleans() {
    ///     println!("{}", b);
    /// }
    /// #
    /// #    Ok(())
    /// # }
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
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// for b in term.ext_booleans() {
    ///     println!("{}", b);
    /// }
    /// #
    /// #    Ok(())
    /// # }
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
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// for b in term.numerics() {
    ///     println!("{}", b);
    /// }
    /// #
    /// #    Ok(())
    /// # }
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
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// for b in term.ext_numerics() {
    ///     println!("{}", b);
    /// }
    /// #
    /// #    Ok(())
    /// # }
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
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// for b in term.strings() {
    ///     println!("{}", b);
    /// }
    /// #
    /// #    Ok(())
    /// # }
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
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use unibilium::Term;
    /// let term_name = "vt100";
    ///
    /// let term = Term::from_term_name(term_name)?;
    /// for b in term.ext_strings() {
    ///     println!("{}", b);
    /// }
    /// #
    /// #    Ok(())
    /// # }
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
