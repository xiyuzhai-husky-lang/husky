use crate::*;

use husky_dev_utils::dev_src;
use husky_path::{FileError, FileErrorKind, FileResultArc};

use husky_source_path::SourcePath;
use husky_text::TextDb;
use husky_tokenize::TokenizeDb;
use husky_word::WordDb;
use salsa::DbWithJar;

#[salsa::jar(db = TokenTextDb)]
pub struct TokenTextJar(tokenized_text);

pub trait TokenTextDb: DbWithJar<TokenTextJar> + TextDb + TokenizeDb {
    fn tokenized_text(&self, id: SourcePath) -> FileResultArc<TokenizedText>;
}

impl<T> TokenTextDb for T
where
    T: DbWithJar<TokenTextJar> + TextDb + TokenizeDb,
{
    fn tokenized_text(&self, id: SourcePath) -> FileResultArc<TokenizedText> {
        todo!()
    }
}

#[salsa::tracked(jar = TokenTextJar)]
fn tokenized_text(db: &dyn TokenTextDb, file: SourcePath) -> FileResultArc<TokenizedText> {
    todo!()
    // if let Some(text) = this.raw_text(file) {
    //     return Ok(TokenizedText::parse(this.word_itr(), text.as_str()));
    // } else {
    //     Err(FileError {
    //         kind: FileErrorKind::FileNotFound,
    //         dev_src: dev_src!(),
    //     })
    // }
}
