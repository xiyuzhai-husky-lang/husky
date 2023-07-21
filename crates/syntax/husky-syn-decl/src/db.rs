use crate::*;
use husky_coword::Ident;
use husky_entity_tree::{EntitySynTreeResult, ImplBlockSynNode};
use husky_print_utils::p;
use husky_vfs::ModulePath;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait DeclDb: DbWithJar<SynDeclJar> + ExprDb {
    fn node_decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<NodeDeclSheet>;

    fn decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<DeclSheet>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<SynDeclJar> + ExprDb,
{
    fn node_decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<NodeDeclSheet> {
        node_decl_sheet(self, module_path)
    }

    fn decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<DeclSheet> {
        decl_sheet(self, module_path)
    }
}
