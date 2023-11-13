use crate::*;

use husky_entity_path::EntityPathDb;
use husky_token::TokenDb;
use husky_vfs::{*, error::VfsResult};

pub trait AstDb: DbWithJar<AstJar> + TokenDb + EntityPathDb {
    #[deprecated(note = "use HasAstSheet trait instead")]
    fn ast_sheet(&self, module_path: ModulePath) -> VfsResult<&AstSheet>;
    #[deprecated(note = "use HasAstSheet trait instead")]
    fn ast_token_idx_range_sheet(
        &self,
        module_path: ModulePath,
    ) -> VfsResult<&AstTokenIdxRangeSheet>;
}

impl<T> AstDb for T
where
    T: DbWithJar<AstJar> + TokenDb + EntityPathDb,
{
    fn ast_sheet(&self, module_path: ModulePath) -> VfsResult<&AstSheet> {
        Ok(ast_sheet(self, module_path).as_ref()?)
    }

    fn ast_token_idx_range_sheet(
        &self,
        module_path: ModulePath,
    ) -> VfsResult<&AstTokenIdxRangeSheet> {
        Ok(ast_token_idx_range_sheet(self, module_path).as_ref()?)
    }
}
