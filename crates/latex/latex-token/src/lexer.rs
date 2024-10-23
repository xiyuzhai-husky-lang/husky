use crate::{
    data::{
        code::LxCodeTokenData,
        math::{LxMathTokenData, LxMathTokenError},
        rose::LxRoseTokenData,
        LxTokenData,
    },
    idx::LxTokenIdx,
    storage::LxTokenStorage,
};
use husky_coword::Coword;
use husky_text_protocol::{char::TextCharIter, range::TextRange};
use latex_prelude::mode::LxMode;

pub struct LxLexer<'a> {
    pub(crate) db: &'a ::salsa::Db,
    pub(crate) chars: TextCharIter<'a>,
    pub(crate) mode: LxMode,
    pub(crate) storage: LxTokenStorage,
}

/// # constructor
impl<'a> LxLexer<'a> {
    pub fn new(db: &'a ::salsa::Db, input: &'a str, mode: LxMode) -> Self {
        Self {
            db,
            chars: TextCharIter::new(input),
            mode,
            storage: Default::default(),
        }
    }
}

/// # actions
impl<'a> LxLexer<'a> {
    pub(crate) fn next_coword_with(&mut self, predicate: impl Fn(char) -> bool) -> Option<Coword> {
        let coword_str_slice = self.chars.next_str_slice_while(|c| c.is_alphanumeric());
        if coword_str_slice.is_empty() {
            return None;
        }
        Some(Coword::from_ref(self.db, coword_str_slice))
    }
}

impl<'a> Iterator for LxLexer<'a> {
    type Item = (LxTokenIdx, LxTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.eat_chars_while(|c| c == ' ');
        let mut start = self.chars.current_position();

        let token_data = if self.chars.eat_char_if(|c| c == '\n') {
            match self.mode {
                LxMode::Code => {
                    self.chars.eat_chars_while(|c| c == '\n');
                    LxCodeTokenData::NewParagraph.into()
                }
                LxMode::Rose => {
                    if self.chars.eat_char_if(|c| c == '\n') {
                        self.chars.eat_chars_while(|c| c == '\n');
                        LxRoseTokenData::NewParagraph.into()
                    } else {
                        start = self.chars.current_position();
                        self.next_token_data()?
                    }
                }
                LxMode::Math => {
                    if self.chars.eat_char_if(|c| c == '\n') {
                        LxMathTokenData::Error(LxMathTokenError::UnexpectedNewParagraph).into()
                    } else {
                        start = self.chars.current_position();
                        self.next_token_data()?
                    }
                }
            }
        } else {
            self.next_token_data()?
        };
        let range = TextRange {
            start,
            end: self.chars.current_position(),
        };
        Some((self.storage.alloc(range, token_data), token_data))
    }
}
