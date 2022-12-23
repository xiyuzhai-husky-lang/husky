use super::*;

// todo: change this to RawTokenVariant
pub(crate) fn new_reserved_word(db: &dyn TokenDb, word: &str) -> Option<RawTokenVariant> {
    Some(reserved_words(db).get_entry(word)?.1.clone())
}
