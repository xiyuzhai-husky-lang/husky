mod special;
mod token;

pub use special::Special;
pub use token::Token;

use std::sync::Arc;

use common::*;

use token::{TokenScanner, TokenizedLine};

#[salsa::query_group(LexQueryStorage)]
pub trait LexQuery: file::FileQuery + word::InternWord {
    fn tokenized_text(&self, id: file::FileId) -> Arc<Result<TokenizedText, FileError>>;
}

fn tokenized_text(this: &dyn LexQuery, id: file::FileId) -> Arc<Result<TokenizedText, FileError>> {
    if let Some(text) = this.text(id) {
        return Arc::new(Ok(TokenizedText::lex(this, text.as_str())));
    } else {
        return Arc::new(Err(FileError::TextNonExistent));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileError {
    TextNonExistent,
}
#[derive(Debug, PartialEq, Eq)]
pub enum LexError {
    IncorrectIndent,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedText {
    tokens: Vec<Token>,
    tokenized_lines: Vec<TokenizedLine>,
    line_groups: Vec<Range>,
    errors: Vec<LexError>,
}

impl TokenizedText {
    fn lex(db: &dyn LexQuery, text: &str) -> Self {
        let mut token_scanner = TokenScanner::new(db);
        text.lines()
            .enumerate()
            .for_each(|(i, line)| token_scanner.scan(i, line));
        token_scanner.into()
    }
}
