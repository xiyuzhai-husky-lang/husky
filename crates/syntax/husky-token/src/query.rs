use crate::{raw_token_iter::RawTokenIter, *};

use husky_dev_utils::dev_src;
use husky_file::{FileError, FileErrorKind, FileResultArc};
use husky_word::InternWord;
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenizedTextQueryGroup: husky_file::FileQueryGroup + Tokenize {
    fn tokenized_text(&self, id: husky_file::FilePtr) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenizedTextQueryGroup,
    id: husky_file::FilePtr,
) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.raw_text(id) {
        return Ok(TokenizedText::parse(this.word_allocator(), text.as_str()));
    } else {
        Err(FileError {
            kind: FileErrorKind::FileNotFound,
            dev_src: dev_src!(),
        })
    }
}

pub trait Tokenize: InternWord {
    fn tokenize_line(&self, line: &str) -> Vec<Token> {
        let mut scanner = TokenScanner::new(self.word_allocator());
        scanner.scan(0, line);
        scanner.finish_with_tokens()
    }
}

impl<T> Tokenize for T where T: InternWord {}
