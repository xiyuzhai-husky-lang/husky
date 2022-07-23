use crate::{line_token_iter::LineTokenIter, *};

use husky_dev_utils::dev_src;
use husky_file::{FileError, FileErrorKind, FileResultArc};
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenSalsaQueryGroup: husky_file::FileQueryGroup + word::InternWord {
    fn tokenized_text(&self, id: husky_file::FilePtr) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenSalsaQueryGroup,
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

pub trait TokenQueryGroup: TokenSalsaQueryGroup {
    fn tokenize(&self, line: &str) -> Vec<HuskyToken> {
        LineTokenIter::new(
            self.word_allocator(),
            0,
            line.chars().enumerate().peekable(),
        )
        .1
        .collect()
    }
}
