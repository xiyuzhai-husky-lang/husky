use crate::*;
use husky_entity_tree::{EntityTreeDb, EntityTreeResult};
use husky_vfs::{ModulePath, VfsResult};
use salsa::DbWithJar;

pub trait DeclDb: DbWithJar<DeclJar> + ExprDb {
    fn module_item_decl(&self, path: ModuleItemPath) -> EntityTreeResult<Decl>;
    fn module_decl_sheet(&self, path: ModulePath) -> EntityTreeResult<&DeclSheet>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + ExprDb,
{
    fn module_item_decl(&self, path: ModuleItemPath) -> EntityTreeResult<Decl> {
        module_item_decl(self, path)
    }
    fn module_decl_sheet(&self, path: ModulePath) -> EntityTreeResult<&DeclSheet> {
        Ok(module_decl_sheet(self, path)
            .as_ref()
            .map_err(|e| e.clone())?)
    }
}
