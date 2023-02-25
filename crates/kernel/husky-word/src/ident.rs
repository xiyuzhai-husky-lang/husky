use crate::*;

use salsa::DebugWithDb;
use std::sync::Arc;
use vec_like::{VecMap, VecPairMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Identifier(Word);

impl Identifier {
    pub fn word(self) -> Word {
        self.0
    }

    pub fn from_owned(db: &dyn WordDb, data: String) -> Option<Self> {
        if is_valid_ident(&data) {
            Some(Self(db.it_word_owned(data)))
        } else {
            None
        }
    }

    pub fn from_borrowed(db: &dyn WordDb, data: &str) -> Option<Self> {
        if is_valid_ident(data) {
            Some(Self(db.it_word_borrowed(data)))
        } else {
            None
        }
    }

    pub fn data(self, db: &dyn WordDb) -> &str {
        db.dt_word(self.0)
    }

    pub fn case(self, db: &dyn WordDb) -> IdentifierCase {
        let data = self.data(db);
        let mut chars = data.chars();
        let is_first_char_uppercase = chars.next().unwrap().is_uppercase();
        // ad hoc
        match chars.next() {
            Some(second_char) => match (is_first_char_uppercase, second_char.is_uppercase()) {
                (true, true) => IdentifierCase::AllCapital,
                (true, false) => IdentifierCase::PascalCase,
                (false, true) => IdentifierCase::CamelCase,
                (false, false) => IdentifierCase::SnakeCase,
            },
            None => {
                if is_first_char_uppercase {
                    IdentifierCase::SingleCapital
                } else {
                    IdentifierCase::SnakeCase
                }
            }
        }
    }
}

#[non_exhaustive]
pub enum IdentifierCase {
    SingleCapital,
    AllCapital,
    SnakeCase,
    PascalCase,
    CamelCase,
}

pub type IdentMap<T> = VecMap<T>;
pub type IdentArcDict<T> = VecMap<Arc<T>>;
pub type IdentPairMap<T> = VecPairMap<Identifier, T>;
#[test]
fn test_is_valid_ident() {
    assert_eq!(is_valid_ident("a"), true);
    assert_eq!(is_valid_ident("b"), true);
    assert_eq!(is_valid_ident("c"), true);
    assert_eq!(is_valid_ident("d"), true);
    assert_eq!(is_valid_ident("e"), true);
    assert_eq!(is_valid_ident("g"), true);
    assert_eq!(is_valid_ident("h"), true);
    assert_eq!(is_valid_ident("i"), true);
    assert_eq!(is_valid_ident("j"), true);
    assert_eq!(is_valid_ident("a1"), true);
    assert_eq!(is_valid_ident("a2"), true);
    assert_eq!(is_valid_ident("a3"), true);
    assert_eq!(is_valid_ident("_a1"), true);
    assert_eq!(is_valid_ident("_1"), true);
    assert_eq!(is_valid_ident("_"), true);
    assert_eq!(is_valid_ident("9"), false);
    assert_eq!(is_valid_ident("9a"), false);
    assert_eq!(is_valid_ident(" "), false);
    assert_eq!(is_valid_ident("*"), false);
    assert_eq!(is_valid_ident("&"), false);
    assert_eq!(is_valid_ident(":"), false);
    assert_eq!(is_valid_ident("{"), false);
    assert_eq!(is_valid_ident("}"), false);
}

pub(crate) fn is_valid_ident(word: &str) -> bool {
    let mut chars = word.chars();
    if let Some(start) = chars.next() {
        if !(start.is_alphabetic() || start == '_') {
            return false;
        }
    }
    for c in chars {
        if !(c.is_alphanumeric() || c == '_') {
            return false;
        }
    }
    true
}

impl<Db: WordDb + ?Sized> DebugWithDb<Db> for Identifier {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<WordJar>>::as_jar_db(db);
        if level.is_root() {
            f.debug_tuple("Identifier").field(&self.data(db)).finish()
        } else {
            f.write_fmt(format_args!("`{}`", &self.data(db)))
        }
    }
}
