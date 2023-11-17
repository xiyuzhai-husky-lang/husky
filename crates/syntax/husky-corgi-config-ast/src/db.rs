use crate::*;
use husky_vfs::{error::VfsResult, VirtualPath};

pub trait CorgiConfigAstDb: salsa::DbWithJar<CorgiConfigAstJar> + TomlAstDb {
    fn corgi_config_ast_sheet(&self, path: VirtualPath) -> VfsResult<Option<&CorgiConfigAstSheet>>;
}

impl<Db> CorgiConfigAstDb for Db
where
    Db: salsa::DbWithJar<CorgiConfigAstJar> + TomlAstDb,
{
    fn corgi_config_ast_sheet(&self, path: VirtualPath) -> VfsResult<Option<&CorgiConfigAstSheet>> {
        match corgi_config_ast_sheet(self, path) {
            Ok(Some(ast_sheet)) => Ok(Some(ast_sheet)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.clone()),
        }
    }
}
