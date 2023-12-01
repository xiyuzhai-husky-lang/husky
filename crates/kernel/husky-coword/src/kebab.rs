use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
#[salsa::deref_id]
pub struct Kebab(Coword);

impl Kebab {
    pub fn coword(self) -> Coword {
        self.0
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        kebab_to_ident(db, self.0)
    }

    pub(crate) unsafe fn from_coword_unchecked(coword: Coword) -> Self {
        Self(coword)
    }

    pub fn from_coword(db: &::salsa::Db, coword: Coword) -> Option<Self> {
        is_coword_valid_kebab(db, coword).then_some(Kebab(coword))
    }

    pub fn from_owned(db: &::salsa::Db, data: String) -> Option<Self> {
        if is_str_valid_kebab(&data) {
            Some(Self(Coword::from_owned(db, data)))
        } else {
            None
        }
    }

    pub fn from_ref(db: &::salsa::Db, data: &str) -> Option<Self> {
        if is_str_valid_kebab(data) {
            Some(Self(Coword::from_ref(db, data)))
        } else {
            None
        }
    }
}

/// only use in this module
#[salsa::tracked(jar = CowordJar)]
pub(crate) fn kebab_to_ident(db: &::salsa::Db, coword: Coword) -> Ident {
    let kebab = coword.data(db);
    if !kebab.contains("-") {
        return Ident::from_ref(db, kebab).unwrap();
    } else {
        Ident::from_owned(db, kebab.replace("-", "_")).unwrap()
    }
}

#[salsa::tracked(jar = CowordJar)]
pub fn is_coword_valid_kebab(db: &::salsa::Db, coword: Coword) -> bool {
    is_str_valid_kebab(coword.data(db))
}

pub fn is_str_valid_kebab(coword: &str) -> bool {
    let mut chars = coword.chars();
    if let Some(start) = chars.next() {
        if !is_char_valid_kebab_first_char(start) {
            return false;
        }
    }
    for c in chars {
        if !is_char_valid_kebab_nonfirst_char(c) {
            return false;
        }
    }
    true
}

pub fn is_char_valid_kebab_first_char(c: char) -> bool {
    // ad hoc
    c.is_alphabetic() || c == '-'
}

pub fn is_char_valid_kebab_nonfirst_char(c: char) -> bool {
    // ad hoc
    c.is_alphanumeric() || c == '-'
}

impl salsa::DebugWithDb for Kebab {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data(db)))
    }
}
