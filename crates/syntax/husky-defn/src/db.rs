use crate::*;
use husky_ast::AstSheet;
use husky_entity_tree::EntityTreeDb;
use husky_vfs::VfsResult;
use salsa::DbWithJar;

pub trait DefnDb: DbWithJar<DefnJar> + DeclDb {
    fn collect_defns<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DefnSheet<'a>>;
}

impl<Db> DefnDb for Db
where
    Db: DbWithJar<DefnJar> + DeclDb,
{
    fn collect_defns<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DefnSheet<'a>> {
        collect_defn_sheet(self, module_path)
    }
}
