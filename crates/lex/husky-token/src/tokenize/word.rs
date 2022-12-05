use super::*;

// todo: change this to RawTokenVariant
pub(crate) fn new_reserved_word(word: &str) -> Option<RawTokenVariant> {
    RESERVED_WORDS.iter().find_map(|(word0, token_kind)| {
        if *word0 == word {
            Some(token_kind.clone())
        } else {
            None
        }
    })
}
