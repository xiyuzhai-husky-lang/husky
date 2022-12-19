use crate::*;
use husky_text::Text;
use husky_token::TokenDb;
use husky_vfs::*;
use idx_arena::map::ArenaKeyQuery;
use std::fmt::Write;
use std::sync::Arc;
use upcast::Upcast;

pub trait AstDb: DbWithJar<AstJar> + TokenDb {
    fn ast_sheet(&self, module: EntityPath) -> &VfsResult<AstSheet>;
    fn ast_range_sheet(&self, module: EntityPath) -> &VfsResult<AstRangeSheet>;
}

impl<T> AstDb for T
where
    T: DbWithJar<AstJar> + TokenDb,
{
    fn ast_sheet(&self, module: EntityPath) -> &VfsResult<AstSheet> {
        ast_sheet(self, module)
    }

    fn ast_range_sheet(&self, module: EntityPath) -> &VfsResult<AstRangeSheet> {
        ast_range_sheet(self, module)
    }
}
