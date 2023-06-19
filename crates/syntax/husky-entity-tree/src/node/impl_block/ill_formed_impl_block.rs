use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct IllFormedImplBlockNodeId {
    path: IllFormedImplBlockPath,
}

impl IllFormedImplBlockNodeId {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.path.module_path(db)
    }

    // pub fn item_node_ids(self, db: &dyn EntityTreeDb) -> &[TraitForTypeItemNodeId] {
    //     todo!()
    // }
}

impl From<IllFormedImplBlockNodeId> for EntityNodeId {
    fn from(id: IllFormedImplBlockNodeId) -> Self {
        EntityNodeId::ImplBlock(id.into())
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedImplBlockNode {
    #[id]
    pub node_id: IllFormedImplBlockNodeId,
    pub impl_token: ImplToken,
    pub ast_idx: AstIdx,
    pub items: Option<ImplBlockItems>,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlockNode {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        impl_token: ImplToken,
        module: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        IllFormedImplBlockNode::new_inner(
            db,
            IllFormedImplBlockNodeId {
                path: IllFormedImplBlockPath::new(db, registry, module),
            },
            impl_token,
            ast_idx,
            items,
            ill_form,
        )
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum ImplBlockIllForm {
    #[error("unmatched angle bras")]
    UnmatchedAngleBras,
    #[error("token error")]
    Token(#[from] TokenError),
    #[error("principal path expr error")]
    MajorPath(#[from] MajorPathExprError),
    #[error("MissingFor")]
    MissingForKeyword,
    #[error("ExpectTypePathAfterFor")]
    ExpectTypePathAfterForKeyword,
}
