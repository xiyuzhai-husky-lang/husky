use crate::{line_token_iter::LineTokenIter, *};

use file::{FileError, FileResultArc};
use std::sync::Arc;
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenSalsaQueryGroup: file::FileQuery + word::InternWord {
    fn tokenized_text(&self, id: file::FileId) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenSalsaQueryGroup,
    id: file::FileId,
) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.text(id) {
        return Ok(Arc::new(TokenizedText::parse(
            this.word_interner(),
            text.as_str(),
        )));
    } else {
        Err(FileError::FileNotFound)
    }
}

pub trait TokenQueryGroup: TokenSalsaQueryGroup {
    fn tokenize(&self, code: &'static str) -> Vec<Token> {
        LineTokenIter::new(self.word_interner(), 0, code.chars().enumerate().peekable())
            .1
            .collect()
    }
}
