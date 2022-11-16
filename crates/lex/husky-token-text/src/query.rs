use crate::*;

use husky_dev_utils::dev_src;
use husky_path::{FileError, FileErrorKind, FileResultArc, PathItd};

use husky_tokenize::Tokenize;
use husky_word::InternWord;
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenizedTextQueryGroup: husky_path::FileQueryGroup + Tokenize {
    fn tokenized_text(&self, id: PathItd) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenizedTextQueryGroup,
    file: PathItd,
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
