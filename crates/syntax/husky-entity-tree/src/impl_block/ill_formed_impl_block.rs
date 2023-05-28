use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedImplBlock {
    #[id]
    pub id: IllFormedImplBlockId,
    pub impl_token: ImplToken,
    pub ast_idx: AstIdx,
    pub items: Option<ImplBlockItems>,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlock {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        impl_token: ImplToken,
        module: ModulePath,
        ast_idx: AstIdx,
        items: Option<ImplBlockItems>,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        IllFormedImplBlock::new_inner(
            db,
            IllFormedImplBlockId {
                module,
                disambiguator: registry.issue_disambiguitor(module, ImplBlockKind::Err),
            },
            impl_token,
            ast_idx,
            items,
            ill_form,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct IllFormedImplBlockId {
    module: ModulePath,
    disambiguator: u8,
}

impl IllFormedImplBlockId {
    pub fn module(self) -> ModulePath {
        self.module
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
