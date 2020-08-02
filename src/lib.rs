use unibilium_sys::{unibi_boolean, unibi_from_env, unibi_term};

pub struct UnibiTerm {
    term: *mut unibi_term,
}

pub struct UnibiTermIter<'a> {
    term: &'a UnibiTerm,
    item: unibi_boolean,
}

impl UnibiTerm {
    pub fn from_env() -> UnibiTerm {
        UnibiTerm {
            term: unsafe { unibi_from_env() },
        }
    }

    pub fn iter(&self) -> UnibiTermIter {
        UnibiTermIter {
            term: self,
            item: unibi_boolean(unibi_boolean::unibi_boolean_begin_.0 + 1),
        }
    }
}

impl<'a> Iterator for UnibiTermIter<'a> {
    type Item = (unibi_boolean, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.item == unibi_boolean::unibi_boolean_end_ {
            return None;
        }

        let res = unsafe { unibilium_sys::unibi_get_bool(self.term.term, self.item) };
        let res = if res > 0 {
            Some((self.item, true))
        } else {
            Some((self.item, false))
        };
        self.item.0 += 1;
        res
    }
}
