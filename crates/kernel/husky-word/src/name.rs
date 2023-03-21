use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Name(Word);

impl Name {
    pub fn word(self) -> Word {
        self.0
    }

    pub fn ident(self, db: &dyn WordDb) -> Ident {
        name_to_ident(db, self.0)
    }

    pub fn from_word(db: &dyn WordDb, word: Word) -> Option<Self> {
        is_word_valid_name(db, word).then_some(Name(word))
    }

    pub fn from_owned(db: &dyn WordDb, data: String) -> Option<Self> {
        if is_str_valid_name(&data) {
            Some(Self(db.it_word_owned(data)))
        } else {
            None
        }
    }

    pub fn from_borrowed(db: &dyn WordDb, data: &str) -> Option<Self> {
        if is_str_valid_name(data) {
            Some(Self(db.it_word_borrowed(data)))
        } else {
            None
        }
    }

    pub fn data(self, db: &dyn WordDb) -> &str {
        db.dt_word(self.0)
    }
}

/// only use in this module
#[salsa::tracked(jar = WordJar)]
pub(crate) fn name_to_ident(db: &dyn WordDb, word: Word) -> Ident {
    let mut name = word.data(db);
    if !name.contains("-") {
        return Ident::from_borrowed(db, name).unwrap();
    } else {
        Ident::from_owned(db, name.replace("-", "_")).unwrap()
    }
}

#[salsa::tracked(jar = WordJar)]
pub fn is_word_valid_name(db: &dyn WordDb, word: Word) -> bool {
    is_str_valid_name(word.data(db))
}

pub fn is_str_valid_name(word: &str) -> bool {
    let mut chars = word.chars();
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

impl<Db: WordDb + ?Sized> salsa::DebugWithDb<Db> for Name {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<WordJar>>::as_jar_db(db);
        if level.is_root() {
            f.debug_tuple("Name").field(&self.data(db)).finish()
        } else {
            f.write_fmt(format_args!("`{}`", self.data(db)))
        }
    }
}
