use crate::{
    data::{
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

pub fn lex_latex_input(input: &str, mode: LxMode, db: &::salsa::Db) -> LxTokenStorage {
    let mut lexer = LxLexer::new(db, input, mode);
    for _ in &mut lexer {}
    lexer.finish()
}

pub(crate) struct LxLexer<'a> {
    pub(crate) db: &'a ::salsa::Db,
    pub(crate) chars: TextCharIter<'a>,
    pub(crate) mode: LxMode,
    pub(crate) storage: LxTokenStorage,
}

/// # constructor
impl<'a> LxLexer<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, input: &'a str, mode: LxMode) -> Self {
        Self {
            db,
            chars: TextCharIter::new(input),
            mode,
            storage: Default::default(),
        }
    }

    pub(crate) fn finish(self) -> LxTokenStorage {
        self.storage
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
    type Item = (LxTokenIdx, (usize, usize), TextRange, LxTokenData);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.eat_chars_while(|c| c == ' ');
        let mut start_offset = self.chars.current_offset();
        let mut start_position = self.chars.current_position();

        let token_data = if self.chars.eat_char_if(|c| c == '\n') {
            self.chars.eat_chars_while(|c| c == ' ');
            match self.mode {
                LxMode::Rose => {
                    if self.chars.eat_char_if(|c| c == '\n') {
                        self.chars.eat_chars_while(|c| c == '\n' || c == ' ');
                        LxRoseTokenData::NewParagraph.into()
                    } else {
                        start_offset = self.chars.current_offset();
                        start_position = self.chars.current_position();
                        self.next_token_data()?
                    }
                }
                LxMode::Math => {
                    if self.chars.eat_char_if(|c| c == '\n') {
                        LxMathTokenData::Error(LxMathTokenError::UnexpectedNewParagraph).into()
                    } else {
                        start_offset = self.chars.current_offset();
                        start_position = self.chars.current_position();
                        self.next_token_data()?
                    }
                }
            }
        } else {
            self.next_token_data()?
        };
        let end_offset = self.chars.current_offset();
        let range = TextRange {
            start: start_position,
            end: self.chars.current_position(),
        };
        Some((
            self.storage
                .alloc(start_offset, end_offset, range, token_data),
            (start_offset, end_offset),
            range,
            token_data,
        ))
    }
}
