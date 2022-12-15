use crate::*;

use husky_dev_utils::dev_src;

use husky_entity_path::{EntityPath, EntityPathDb};
use husky_source_path::{SourcePath, SourcePathData};
use husky_text::TextCharIter;
use husky_vfs::{VfsDb, VfsResult};
use husky_word::WordDb;
use salsa::DbWithJar;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(token_sheet);

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb + EntityPathDb {
    fn token_sheet(&self, entity_path: EntityPath) -> &VfsResult<TokenGroupSheet>;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb + EntityPathDb,
{
    fn token_sheet(&self, entity_path: EntityPath) -> &VfsResult<TokenGroupSheet> {
        token_sheet(self, entity_path)
    }
}

#[salsa::tracked(jar = TokenJar,return_ref)]
fn token_sheet(db: &dyn TokenDb, entity_path: EntityPath) -> VfsResult<TokenGroupSheet> {
    Ok(TokenGroupSheet::new(tokenize::tokenize(
        db.word_db(),
        db.source_text(db.it_source_path(SourcePathData::Module(entity_path)))?,
    )))
}
