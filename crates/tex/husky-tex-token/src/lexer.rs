use crate::{data::TexTokenData, idx::TexTokenIdx, storage::TexTokenStorage, *};
use husky_coword::Coword;
use husky_tex_prelude::mode::TexMode;
use husky_text_protocol::{char_iter::TextCharIter, range::TextRange};

pub struct TexLexer<'a> {
    pub(crate) db: &'a ::salsa::Db,
    pub(crate) chars: TextCharIter<'a>,
    pub(crate) mode: TexMode,
    pub(crate) storage: TexTokenStorage,
}

/// # constructor
impl<'a> TexLexer<'a> {
    pub fn new(db: &'a ::salsa::Db, input: &'a str, mode: TexMode) -> Self {
        Self {
            db,
            chars: TextCharIter::new(input),
            mode,
            storage: Default::default(),
        }
    }
}

/// # actions
impl<'a> TexLexer<'a> {
    pub(crate) fn next_coword_with(&mut self, predicate: impl Fn(char) -> bool) -> Option<Coword> {
        let coword_str_slice = self.chars.next_str_slice_with(|c| c.is_alphanumeric());
        if coword_str_slice.is_empty() {
            return None;
        }
        Some(Coword::from_ref(self.db, coword_str_slice))
    }
}

impl<'a> Iterator for TexLexer<'a> {
    type Item = (TexTokenIdx, TexTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.chars.current_position();
        let token_data = self.next_token_data()?;
        let range = TextRange {
            start,
            end: self.chars.current_position(),
        };
        Some((self.storage.alloc(range, token_data), token_data))
    }
}
