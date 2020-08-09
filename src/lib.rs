use std::ffi::CStr;
use unibilium_sys::{
    unibi_boolean, unibi_from_env, unibi_from_term, unibi_numeric, unibi_string, unibi_term,
};

pub struct UnibiTerm {
    term: *mut unibi_term,
}

pub struct UnibiTermBoolIter<'a> {
    term: &'a UnibiTerm,
    item: unibi_boolean,
}

pub struct UnibiTermNumericIter<'a> {
    term: &'a UnibiTerm,
    item: unibi_numeric,
}

impl UnibiTerm {
    pub fn from_env() -> UnibiTerm {
        UnibiTerm {
            term: unsafe { unibi_from_env() },
        }
    }

    pub fn from_term_name(name: &str) -> UnibiTerm {
        UnibiTerm {
            term: unsafe { unibi_from_term(name.as_ptr() as *const i8) },
        }
    }

    pub fn booleans(&self) -> Vec<(unibi_boolean, bool)> {
        let mut all = vec![];
        let first = unibi_boolean::unibi_boolean_begin_.0 + 1;
        let end = unibi_boolean::unibi_boolean_end_.0;
        for current in first..end {
            let b = unibi_boolean(current);
            let value = unsafe { unibilium_sys::unibi_get_bool(self.term, b) };
            let value = value > 0;
            let value = (b, value);
            all.push(value);
        }
        all
    }

    pub fn ext_booleans(&self) -> Vec<(Option<String>, bool)> {
        let mut all = vec![];
        let end = unsafe { unibilium_sys::unibi_count_ext_bool(self.term) };
        for current in 0..end {
            let key = unsafe { unibilium_sys::unibi_get_ext_bool_name(self.term, current) };
            let key = self.c_char_to_string(key);
            let value = unsafe { unibilium_sys::unibi_get_ext_bool(self.term, current) };
            let value = value > 0;
            let value = (key, value);
            all.push(value);
        }
        all
    }

    pub fn numerics(&self) -> Vec<(unibi_numeric, i32)> {
        let mut all = vec![];
        let first = unibi_numeric::unibi_numeric_begin_.0 + 1;
        let end = unibi_numeric::unibi_numeric_end_.0;
        for current in first..end {
            let b = unibi_numeric(current);
            let value = unsafe { unibilium_sys::unibi_get_num(self.term, b) };
            let value = (b, value);
            all.push(value);
        }
        all
    }

    fn c_char_to_string(&self, ptr: *const i8) -> Option<String> {
        if ptr.is_null() {
            None
        } else {
            let value = unsafe { CStr::from_ptr(ptr) };
            let value = value.to_string_lossy().into_owned();
            Some(value)
        }
    }

    pub fn strings(&self) -> Vec<(unibi_string, Option<String>)> {
        let mut all = vec![];
        let first = unibi_string::unibi_string_begin_.0 + 1;
        let end = unibi_string::unibi_string_end_.0;
        for current in first..end {
            let s = unibi_string(current);
            let value = unsafe { unibilium_sys::unibi_get_str(self.term, s) };
            all.push((s, self.c_char_to_string(value)));
        }
        all
    }

    pub fn iter_bool(&self) -> UnibiTermBoolIter {
        UnibiTermBoolIter {
            term: self,
            item: unibi_boolean(unibi_boolean::unibi_boolean_begin_.0 + 1),
        }
    }

    pub fn iter_numeric(&self) -> UnibiTermNumericIter {
        UnibiTermNumericIter {
            term: self,
            item: unibi_numeric(unibi_numeric::unibi_numeric_begin_.0 + 1),
        }
    }
}

impl Drop for UnibiTerm {
    fn drop(&mut self) {
        unsafe {
            unibilium_sys::unibi_destroy(self.term);
        }
    }
}

impl<'a> Iterator for UnibiTermBoolIter<'a> {
    type Item = (unibi_boolean, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.item == unibi_boolean::unibi_boolean_end_ {
            return None;
        }

        let res = unsafe { unibilium_sys::unibi_get_bool(self.term.term, self.item) };
        let res = res > 0;
        let res = Some((self.item, res));
        self.item.0 += 1;
        res
    }
}

impl<'a> Iterator for UnibiTermNumericIter<'a> {
    type Item = (unibi_numeric, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.item == unibi_numeric::unibi_numeric_end_ {
            return None;
        }

        let res = unsafe { unibilium_sys::unibi_get_num(self.term.term, self.item) };
        let res = Some((self.item, res));
        self.item.0 += 1;
        res
    }
}
