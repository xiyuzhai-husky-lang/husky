use crate::*;

use husky_vfs::*;
use husky_word::WordDb;
use salsa::DbWithJar;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(token_sheet);

pub trait TokenDb: DbWithJar<TokenJar> + VfsDb {
    fn token_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenSheet>;
}

impl<T> TokenDb for T
where
    T: DbWithJar<TokenJar> + VfsDb,
{
    fn token_sheet(&self, module_path: ModulePath) -> VfsResult<&TokenSheet> {
        Ok(token_sheet(self, module_path).as_ref()?)
    }
}

#[salsa::tracked(jar = TokenJar,return_ref)]
fn token_sheet(db: &dyn TokenDb, module_path: ModulePath) -> VfsResult<TokenSheet> {
    Ok(TokenSheet::new(tokenize::tokenize(
        db.word_db(),
        db.module_content(module_path)?,
    )))
}
