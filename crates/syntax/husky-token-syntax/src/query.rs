use crate::*;

use husky_dev_utils::dev_src;
use husky_file::{FileError, FileErrorKind, FileItd, FileResultArc};

use husky_word::InternWord;
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenizedTextQueryGroup: husky_file::FileQueryGroup + Tokenize {
    fn tokenized_text(&self, id: FileItd) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenizedTextQueryGroup,
    file: FileItd,
) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.raw_text(file) {
        return Ok(TokenizedText::parse(this.word_itr(), text.as_str()));
    } else {
        Err(FileError {
            kind: FileErrorKind::FileNotFound,
            dev_src: dev_src!(),
        })
    }
}

pub trait Tokenize: InternWord {
    fn tokenize_line(&self, line: &str) -> Vec<Token> {
        let mut scanner = TokenScanner::new(self.word_itr());
        scanner.scan_line(0, line);
        scanner.finish_with_tokens()
    }
}

impl<T> Tokenize for T where T: InternWord {}
