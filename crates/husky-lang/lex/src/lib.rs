mod special;
mod token;
mod token_iter;
mod token_scanner;
mod tokenized_text;

pub use special::Special;
pub use token::Token;
pub use tokenized_text::TokenizedText;

use std::sync::Arc;

use common::*;

use file::FileError;
use text::Indent;
use token_scanner::TokenScanner;

#[salsa::query_group(LexQueryStorage)]
pub trait LexQuery: file::FileQuery + word::InternWord {
    fn tokenized_text(&self, id: file::FileId) -> Arc<Result<TokenizedText, FileError>>;
}

fn tokenized_text(this: &dyn LexQuery, id: file::FileId) -> Arc<Result<TokenizedText, FileError>> {
    if let Some(text) = this.text(id) {
        return Arc::new(Ok(TokenizedText::lex(this, text.as_str())));
    } else {
        return Arc::new(Err(FileError::FileNotFound));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexError {
    IncorrectIndent,
}
