use crate::*;
use husky_vfs::{DiffPath, VfsResult};

pub trait CorgiConfigAstDb: salsa::DbWithJar<CorgiConfigAstJar> + TomlAstDb {
    fn corgi_config_ast_sheet(&self, path: DiffPath) -> VfsResult<&CorgiConfigAstSheet>;
}

impl<Db> CorgiConfigAstDb for Db
where
    Db: salsa::DbWithJar<CorgiConfigAstJar> + TomlAstDb,
{
    fn corgi_config_ast_sheet(&self, path: DiffPath) -> VfsResult<&CorgiConfigAstSheet> {
        let file = self.file_from_diff_path(path)?;
        todo!()
    }
}
