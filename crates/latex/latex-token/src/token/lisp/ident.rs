use super::*;
use eterned::db::EternerDb;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LxLispIdent(BaseCoword);

impl LxLispIdent {
    pub fn new(s: &str, db: &EternerDb) -> Self {
        debug_assert!(s.chars().next().unwrap().is_ascii_alphabetic());
        fn is_valid_ident(s: &str) -> bool {
            s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        debug_assert!(is_valid_ident(s));
        Self(BaseCoword::from_ref(s, db))
    }
}
