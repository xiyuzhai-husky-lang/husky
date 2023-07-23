use crate::*;
use husky_coword::Ident;
use husky_entity_tree::{EntitySynTreeResult, ImplBlockSynNode};
use husky_print_utils::p;
use husky_vfs::ModulePath;
use salsa::DbWithJar;
use vec_like::VecMapGetEntry;

pub trait DeclDb: DbWithJar<SynDeclJar> + ExprDb {
    fn syn_node_decl_sheet(&self, module_path: ModulePath)
        -> EntitySynTreeResult<SynNodeDeclSheet>;

    fn syn_decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<SynDeclSheet>;
}

impl<Db> DeclDb for Db
where
    Db: DbWithJar<SynDeclJar> + ExprDb,
{
    fn syn_node_decl_sheet(
        &self,
        module_path: ModulePath,
    ) -> EntitySynTreeResult<SynNodeDeclSheet> {
        syn_node_decl_sheet(self, module_path)
    }

    fn syn_decl_sheet(&self, module_path: ModulePath) -> EntitySynTreeResult<SynDeclSheet> {
        syn_decl_sheet(self, module_path)
    }
}
