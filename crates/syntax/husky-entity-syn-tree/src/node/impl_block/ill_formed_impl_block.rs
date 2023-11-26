use original_error::OriginalError;

use super::*;

#[salsa::interned(db =EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct IllFormedImplBlockSynNodePath {
    module_path: ModulePath,
    disambiguator: u8,
}

impl IllFormedImplBlockSynNodePath {
    pub fn item_syn_node_paths(self, _db: &dyn EntitySynTreeDb) -> &[IllFormedItemSynNodePath] {
        // ad hoc
        &[]
    }

    pub(crate) fn syn_node(self, db: &dyn EntitySynTreeDb) -> IllFormedImplBlockSynNode {
        ill_formed_impl_block_syn_node(db, self)
    }
}

// impl HasModulePath<Db> for IllFormedImplBlockSynNodePath
// where
//      + EntitySynTreeDb,
// {
//     fn module_path(self, db: &Db) -> ModulePath {
//         let db = entity_syn_tree_db(db);
//         // self.path.module_path(db)
//         todo!()
//     }
// }

impl From<IllFormedImplBlockSynNodePath> for ItemSynNodePath {
    fn from(id: IllFormedImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub(crate) struct IllFormedImplBlockSynNode {
    #[id]
    pub syn_node_path: IllFormedImplBlockSynNodePath,
    pub impl_token: ImplToken,
    pub ast_idx: AstIdx,
    pub items: Option<ImplBlockItems>,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlockSynNode {
    pub(super) fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ImplBlockRegistry,
        impl_token: ImplToken,
        module: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        todo!()
        // IllFormedImplBlockSynNode::new_inner(
        //     db,
        //     IllFormedImplBlockSynNodePath {
        //         path: IllFormedImplBlockPath::new(db, registry, module),
        //     },
        //     impl_token,
        //     ast_idx,
        //     items,
        //     ill_form,
        // )
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum ImplBlockIllForm {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    TokenData(#[from] TokenDataError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
    #[error("MissingFor")]
    MissingForKeyword,
    #[error("ExpectTypePathAfterFor")]
    ExpectTypePathAfterForKeyword,
    #[error("expected `derive` identifier")]
    ExpectedDeriveIdent(TokenStreamState),
}

impl OriginalError for ImplBlockIllForm {
    type Error = Self;
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn ill_formed_impl_block_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: IllFormedImplBlockSynNodePath,
) -> IllFormedImplBlockSynNode {
    let module_path = syn_node_path.module_path(db);
    let item_tree_sheet = db.item_syn_tree_sheet(module_path);
    item_tree_sheet.ill_formed_impl_block_syn_node(db, syn_node_path)
}
