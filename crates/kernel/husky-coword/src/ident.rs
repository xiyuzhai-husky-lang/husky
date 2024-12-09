use crate::*;
use salsa::DebugWithDb;
use vec_like::{VecMap, VecPairMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[salsa::as_id_with_db(jar = CowordJar)]
#[salsa::deref_id]
pub struct Ident(BaseCoword);

impl Ident {
    pub fn coword(self) -> BaseCoword {
        self.0
    }

    pub fn from_owned(db: &::salsa::Db, data: String) -> Option<Self> {
        if is_str_valid_ident(&data) {
            Some(Self(BaseCoword::new(data, db)))
        } else {
            None
        }
    }

    pub fn from_ref(db: &::salsa::Db, data: &str) -> Option<Self> {
        if is_str_valid_ident(data) {
            Some(Self(BaseCoword::from_ref(data, db)))
        } else {
            None
        }
    }

    pub fn case(self) -> IdentCase {
        let data = self.data();
        let mut chars = data.chars();
        let is_first_char_uppercase = chars.next().unwrap().is_uppercase();
        // ad hoc
        match chars.next() {
            Some(second_char) => match (is_first_char_uppercase, second_char.is_uppercase()) {
                (true, true) => IdentCase::AllCapital,
                (true, false) => IdentCase::PascalCase,
                (false, true) => IdentCase::CamelCase,
                (false, false) => IdentCase::SnakeCase,
            },
            None => {
                if is_first_char_uppercase {
                    IdentCase::SingleCapital
                } else {
                    IdentCase::SnakeCase
                }
            }
        }
    }
}

pub enum IdentCase {
    SingleCapital,
    AllCapital,
    SnakeCase,
    PascalCase,
    CamelCase,
}

pub type IdentMap<T> = VecMap<T>;
pub type IdentPairMap<T> = VecPairMap<Ident, T>;
#[test]
fn test_is_valid_ident() {
    assert_eq!(is_str_valid_ident("a"), true);
    assert_eq!(is_str_valid_ident("b"), true);
    assert_eq!(is_str_valid_ident("c"), true);
    assert_eq!(is_str_valid_ident("d"), true);
    assert_eq!(is_str_valid_ident("e"), true);
    assert_eq!(is_str_valid_ident("g"), true);
    assert_eq!(is_str_valid_ident("h"), true);
    assert_eq!(is_str_valid_ident("i"), true);
    assert_eq!(is_str_valid_ident("j"), true);
    assert_eq!(is_str_valid_ident("a1"), true);
    assert_eq!(is_str_valid_ident("a2"), true);
    assert_eq!(is_str_valid_ident("a3"), true);
    assert_eq!(is_str_valid_ident("_a1"), true);
    assert_eq!(is_str_valid_ident("_1"), true);
    assert_eq!(is_str_valid_ident("_"), true);
    assert_eq!(is_str_valid_ident("9"), false);
    assert_eq!(is_str_valid_ident("9a"), false);
    assert_eq!(is_str_valid_ident(" "), false);
    assert_eq!(is_str_valid_ident("*"), false);
    assert_eq!(is_str_valid_ident("&"), false);
    assert_eq!(is_str_valid_ident(":"), false);
    assert_eq!(is_str_valid_ident("{"), false);
    assert_eq!(is_str_valid_ident("}"), false);
}

pub fn is_str_valid_ident(word: &str) -> bool {
    let mut chars = word.chars();
    if let Some(start) = chars.next() {
        if !is_char_valid_ident_first_char(start) {
            return false;
        }
    }
    for c in chars {
        if !is_char_valid_ident_nonfirst_char(c) {
            return false;
        }
    }
    true
}

pub fn is_char_valid_ident_first_char(c: char) -> bool {
    // ad hoc
    c.is_alphabetic() || c == '_'
}

pub fn is_char_valid_ident_nonfirst_char(c: char) -> bool {
    // ad hoc
    c.is_alphanumeric() || c == '_'
}

impl std::fmt::Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
    }
}

impl DebugWithDb for Ident {
    fn debug_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_fmt(format_args!("`{}`", self.data()))
    }
}

/// only use in this module
#[salsa::tracked]
pub fn ident_to_name(db: &::salsa::Db, ident: Ident) -> Kebab {
    let ident_data = ident.data();
    if !ident_data.contains("_") {
        return unsafe { Kebab::from_coword_unchecked(ident.0) };
    } else {
        Kebab::from_owned(db, ident_data.replace("_", "-")).unwrap()
    }
}
