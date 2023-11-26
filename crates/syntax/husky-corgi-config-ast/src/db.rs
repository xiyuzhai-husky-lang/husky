use crate::*;
use husky_vfs::{error::VfsResult, VirtualPath};
use salsa::Db;

pub trait CorgiConfigAstDb {
    fn corgi_config_ast_sheet(&self, path: VirtualPath) -> VfsResult<Option<&CorgiConfigAstSheet>>;
}

impl CorgiConfigAstDb for Db {
    fn corgi_config_ast_sheet(&self, path: VirtualPath) -> VfsResult<Option<&CorgiConfigAstSheet>> {
        match corgi_config_ast_sheet(self, path) {
            Ok(Some(ast_sheet)) => Ok(Some(ast_sheet)),
            Ok(None) => Ok(None),
            Err(e) => Err(e.clone()),
        }
    }
}
