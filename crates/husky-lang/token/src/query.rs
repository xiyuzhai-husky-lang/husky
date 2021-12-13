use crate::*;

use file::FileError;
use std::sync::Arc;
#[salsa::query_group(TokenQueryStorage)]
pub trait TokenQuery: file::FileQuery + word::InternWord {
    fn tokenized_text(&self, id: file::FileId) -> Arc<Result<TokenizedText, FileError>>;
}

fn tokenized_text(
    this: &dyn TokenQuery,
    id: file::FileId,
) -> Arc<Result<TokenizedText, FileError>> {
    if let Some(text) = this.text(id) {
        return Arc::new(Ok(TokenizedText::token(this, text.as_str())));
    } else {
        return Arc::new(Err(FileError::FileNotFound));
    }
}
