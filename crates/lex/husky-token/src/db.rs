use crate::*;

use husky_dev_utils::dev_src;

use husky_source_path::SourcePath;
use husky_text::TextCharIter;
use husky_vfs::{VfsDb, VfsResult};
use husky_word::WordDb;
use salsa::DbWithJar;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(tokenized_text);

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb {
    fn token_sheet(&self, id: SourcePath) -> &VfsResult<TokenSheet>;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb,
{
    fn token_sheet(&self, id: SourcePath) -> &VfsResult<TokenSheet> {
        todo!()
    }
}

#[salsa::tracked(jar = TokenJar,return_ref)]
fn tokenized_text(db: &dyn TokenDb, file: SourcePath) -> VfsResult<TokenSheet> {
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
