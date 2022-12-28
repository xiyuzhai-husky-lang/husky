use crate::*;
use husky_entity_tree::{EntityTreeDb, EntityTreeResult};
use husky_vfs::{ModulePath, VfsResult};
use salsa::DbWithJar;

pub trait DeclDb: DbWithJar<DeclJar> + EntityTreeDb {
    fn decl_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&DeclSheet>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + EntityTreeDb,
{
    fn decl_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&DeclSheet> {
        Ok(decl_sheet(self, module_path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
