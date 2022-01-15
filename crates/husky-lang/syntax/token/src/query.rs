use crate::*;

use file::{FileError, FileResultArc};
use std::sync::Arc;
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenQueryGroup: file::FileQuery + word::InternWord {
    fn tokenized_text(&self, id: file::FileId) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(this: &dyn TokenQueryGroup, id: file::FileId) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.text(id) {
        return Ok(Arc::new(TokenizedText::parse(
            this.word_interner(),
            text.as_str(),
        )));
    } else {
        Err(FileError::FileNotFound)
    }
}
