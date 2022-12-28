use crate::*;
use husky_ast::AstSheet;
use husky_entity_tree::EntityTreeDb;
use husky_vfs::VfsResult;
use salsa::DbWithJar;

pub trait DefnDb: DbWithJar<DefnJar> + DeclDb {
    fn defn_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&DefnSheet>;
}

impl<Db> DefnDb for Db
where
    Db: DbWithJar<DefnJar> + DeclDb,
{
    fn defn_sheet(&self, module_path: ModulePath) -> EntityTreeResult<&DefnSheet> {
        Ok(defn_sheet(self, module_path).as_ref()?)
    }
}
