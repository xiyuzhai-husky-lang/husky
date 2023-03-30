use crate::*;
use husky_entity_tree::{AssociatedItem, EntityTreeResult, ImplBlock};
use husky_print_utils::p;
use husky_vfs::ModulePath;
use husky_word::Ident;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait DeclDb: DbWithJar<DeclJar> + ExprDb {
    fn decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<DeclJar> + ExprDb,
{
    fn decl_sheet<'a>(&'a self, module_path: ModulePath) -> EntityTreeResult<DeclSheet<'a>> {
        decl_sheet(self, module_path)
    }
}
