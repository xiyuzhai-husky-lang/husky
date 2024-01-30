use original_error::OriginalError;

use super::*;

#[salsa::debug_with_db]
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
    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a IllFormedImplBlockSynNode {
        let module_path = self.module_path(db);
        let item_tree_sheet = db.item_syn_tree_sheet(module_path);
        item_tree_sheet.ill_formed_impl_block_syn_node(db, self)
    }

    pub(crate) fn associated_items(
        self,
        _db: &::salsa::Db,
    ) -> &[(Ident, IllFormedItemSynNodePath, IllFormedItemSynNode)] {
        // ill_formed_impl_block_items(db, self)
        // ad hoc
        &[]
    }

    pub fn item_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = IllFormedItemSynNodePath> + 'a {
        self.associated_items(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl IllFormedImplBlockSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> IllFormedImplBlockSynNodePath {
        IllFormedImplBlockSynNodePath(id)
    }

    pub fn module_path(self, _db: &::salsa::Db) -> ModulePath {
        self.module_path
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        IllFormedImplBlockSynNodePath(id).syn_node(db).ast_idx
    }
}

impl From<IllFormedImplBlockSynNodePath> for ItemSynNodePath {
    fn from(id: IllFormedImplBlockSynNodePath) -> Self {
        ItemSynNodePath::ImplBlock(id.into())
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct IllFormedImplBlockSynNode {
    syn_node_path: IllFormedImplBlockSynNodePath,
    pub(crate) impl_token: ImplToken,
    pub(crate) ast_idx: AstIdx,
    pub(crate) items: Option<ImplBlockItems>,
    pub(crate) ill_form: ImplBlockIllForm,
}

/// # constructor
impl IllFormedImplBlockSynNode {
    pub(super) fn new(
        db: &::salsa::Db,
        registry: &mut ImplBlockRegistry,
        impl_token: ImplToken,
        module_path: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        IllFormedImplBlockSynNode {
            syn_node_path: IllFormedImplBlockSynNodePath(ItemSynNodePathId::new(
                db,
                ItemSynNodePathData::ImplBlock(ImplBlockSynNodePathData::IllFormedImplBlock(
                    IllFormedImplBlockSynNodePathData {
                        module_path,
                        disambiguator: registry.issue_ill_formed_disambiguitor(module_path),
                    },
                )),
            )),
            impl_token,
            ast_idx,
            items,
            ill_form,
        }
    }
}

/// # getters
impl IllFormedImplBlockSynNode {
    pub(crate) fn syn_node_path(&self) -> IllFormedImplBlockSynNodePath {
        self.syn_node_path
    }

    pub(crate) fn ill_form(&self) -> &ImplBlockIllForm {
        &self.ill_form
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db]
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
    #[error("UnexpectedFugitivePath")]
    UnexpectedFugitivePath(FugitivePath),
    #[error("InvalidTypeSketch")]
    InvalidTypeSketch,
}

impl OriginalError for ImplBlockIllForm {
    type Error = Self;
}
