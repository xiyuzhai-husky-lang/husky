use crate::*;
use husky_coword::Ident;
use husky_entity_tree::{EntityTreeResult, ImplBlockNode};
use husky_print_utils::p;
use husky_vfs::ModulePath;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait DeclDb: DbWithJar<SynDeclJar> + ExprDb {
    fn node_decl_sheet(&self, module_path: ModulePath) -> EntityTreeResult<NodeDeclSheet>;

    fn decl_sheet(&self, module_path: ModulePath) -> EntityTreeResult<DeclSheet>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<SynDeclJar> + ExprDb,
{
    fn node_decl_sheet(&self, module_path: ModulePath) -> EntityTreeResult<NodeDeclSheet> {
        node_decl_sheet(self, module_path)
    }

    fn decl_sheet(&self, module_path: ModulePath) -> EntityTreeResult<DeclSheet> {
        decl_sheet(self, module_path)
    }
}
