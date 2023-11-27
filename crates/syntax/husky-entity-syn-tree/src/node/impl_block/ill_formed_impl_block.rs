use original_error::OriginalError;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct IllFormedImplBlockSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IllFormedImplBlockSynNodePathData {
    module_path: ModulePath,
    disambiguator: u8,
}

impl IllFormedImplBlockSynNodePath {
    pub fn item_syn_node_paths(self, _db: &::salsa::Db) -> &[ItemSynNodePathId] {
        // ad hoc
        &[]
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a IllFormedImplBlockSynNode {
        let module_path = self.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        item_tree_sheet.ill_formed_impl_block_syn_node(db, self)
    }
}

impl From<IllFormedImplBlockSynNodePath> for ItemSynNodePath {
    fn from(id: IllFormedImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct IllFormedImplBlockSynNode {
    pub(crate) syn_node_path: IllFormedImplBlockSynNodePath,
    pub(crate) impl_token: ImplToken,
    pub(crate) ast_idx: AstIdx,
    pub(crate) items: Option<ImplBlockItems>,
    pub(crate) ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlockSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
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

    pub(crate) fn ill_form(&self) -> &ImplBlockIllForm {
        &self.ill_form
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
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
