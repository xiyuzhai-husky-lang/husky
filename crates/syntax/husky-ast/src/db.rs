use crate::*;
use husky_display_utils::HuskyDisplayConfig;
use husky_source_path::SourcePath;
use husky_text::Text;
use husky_token::{AbsSemanticToken, TokenDb};
use husky_vfs::VfsResult;
use idx_arena::map::ArenaKeyQuery;
use std::fmt::Write;
use std::sync::Arc;
use upcast::Upcast;

pub trait AstDb: DbWithJar<AstJar> + TokenDb {
    fn ast_sheet(&self, module: EntityPath) -> &VfsResult<AstSheet>;
}

impl<T> AstDb for T
where
    T: DbWithJar<AstJar> + TokenDb,
{
    fn ast_sheet(&self, module: EntityPath) -> &VfsResult<AstSheet> {
        ast_sheet(self, module)
    }
}
