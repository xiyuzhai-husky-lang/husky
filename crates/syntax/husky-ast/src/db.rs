use crate::*;

use husky_token::TokenDb;
use husky_vfs::*;

pub trait AstDb: DbWithJar<AstJar> + TokenDb {
    fn ast_sheet(&self, module: ModulePath) -> VfsResult<&AstSheet>;
    fn ast_range_sheet(&self, module: ModulePath) -> VfsResult<&AstRangeSheet>;
}

impl<T> AstDb for T
where
    T: DbWithJar<AstJar> + TokenDb,
{
    fn ast_sheet(&self, module: ModulePath) -> VfsResult<&AstSheet> {
        Ok(ast_sheet(self, module).as_ref()?)
    }

    fn ast_range_sheet(&self, module: ModulePath) -> VfsResult<&AstRangeSheet> {
        Ok(ast_range_sheet(self, module).as_ref()?)
    }
}
