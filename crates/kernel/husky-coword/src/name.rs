use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Name(Coword);

impl Name {
    pub fn coword(self) -> Coword {
        self.0
    }

    pub fn ident(self, db: &dyn CowordDb) -> Ident {
        name_to_ident(db, self.0)
    }

    pub(crate) unsafe fn from_coword_unchecked(coword: Coword) -> Self {
        Self(coword)
    }

    pub fn from_coword(db: &dyn CowordDb, coword: Coword) -> Option<Self> {
        is_coword_valid_name(db, coword).then_some(Name(coword))
    }

    pub fn from_owned(db: &dyn CowordDb, data: String) -> Option<Self> {
        if is_str_valid_name(&data) {
            Some(Self(db.it_coword_owned(data)))
        } else {
            None
        }
    }

    pub fn from_ref(db: &dyn CowordDb, data: &str) -> Option<Self> {
        if is_str_valid_name(data) {
            Some(Self(db.it_coword_borrowed(data)))
        } else {
            None
        }
    }

    pub fn data(self, db: &dyn CowordDb) -> &str {
        db.dt_coword(self.0)
    }
}

/// only use in this module
#[salsa::tracked(jar = CowordJar)]
pub(crate) fn name_to_ident(db: &dyn CowordDb, coword: Coword) -> Ident {
    let name = coword.data(db);
    if !name.contains("-") {
        return Ident::from_borrowed(db, name).unwrap();
    } else {
        Ident::from_owned(db, name.replace("-", "_")).unwrap()
    }
}

#[salsa::tracked(jar = CowordJar)]
pub fn is_coword_valid_name(db: &dyn CowordDb, coword: Coword) -> bool {
    is_str_valid_name(coword.data(db))
}

pub fn is_str_valid_name(coword: &str) -> bool {
    let mut chars = coword.chars();
    if let Some(start) = chars.next() {
        if !is_char_valid_name_first_char(start) {
            return false;
        }
    }
    for c in chars {
        if !is_char_valid_name_nonfirst_char(c) {
            return false;
        }
    }
    true
}

pub fn is_char_valid_name_first_char(c: char) -> bool {
    // ad hoc
    c.is_alphabetic() || c == '-'
}

pub fn is_char_valid_name_nonfirst_char(c: char) -> bool {
    // ad hoc
    c.is_alphanumeric() || c == '-'
}

impl<Db: CowordDb + ?Sized> salsa::DebugWithDb<Db> for Name {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<CowordJar>>::as_jar_db(db);
        if level.is_root() {
            f.debug_tuple("Name").field(&self.data(db)).finish()
        } else {
            f.write_fmt(format_args!("`{}`", self.data(db)))
        }
    }
}
