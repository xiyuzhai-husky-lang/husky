use crate::*;
use husky_token::{TokenKind, RESERVED_WORDS};

// todo: change this to RawTokenVariant
pub(crate) fn new_reserved_word(word: &str) -> Option<TokenKind> {
    RESERVED_WORDS.iter().find_map(|(word0, token_kind)| {
        if *word0 == word {
            Some(token_kind.clone())
        } else {
            None
        }
    })
}
