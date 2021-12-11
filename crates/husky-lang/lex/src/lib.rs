mod special;
mod token;

pub use special::Special;
pub use token::Token;

use std::sync::Arc;

use token::{TokenScanner, TokenizedLine};

#[salsa::query_group(LexQueryStorage)]
pub trait LexQuery: file::FileQuery + word::InternWord {
    fn lex_result(&self, id: file::FileId) -> Arc<LexResult>;
}

fn lex_result(this: &dyn LexQuery, id: file::FileId) -> Arc<LexResult> {
    if let Some(text) = this.text(id) {
        return Arc::new(LexResult::TextTokens(TokenizedText::lex(
            this,
            text.as_str(),
        )));
    } else {
        return Arc::new(LexResult::TextNonExistent);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexResult {
    TextNonExistent,
    TextTokens(TokenizedText),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TokenizedText {
    tokens: Vec<Token>,
    tokenized_lines: Vec<TokenizedLine>,
}

impl TokenizedText {
    fn lex(db: &dyn LexQuery, text: &str) -> Self {
        let mut token_scanner = TokenScanner::new(db);
        text.lines().for_each(|line| token_scanner.scan(line));
        token_scanner.into()
    }
}
