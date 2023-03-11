use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct IllFormedImplBlock {
    #[id]
    pub id: IllFormedImplBlockId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

impl IllFormedImplBlock {
    pub(super) fn new(
        db: &dyn EntityTreeDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ast_idx: AstIdx,
        body: AstIdxRange,
        ill_form: ImplBlockIllForm,
    ) -> Self {
        IllFormedImplBlock::new_inner(
            db,
            IllFormedImplBlockId {
                module_path,
                disambiguator: registry.issue_disambiguitor(module_path, ImplBlockKind::Err),
            },
            ast_idx,
            body,
            ill_form,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct IllFormedImplBlockId {
    module_path: ModulePath,
    disambiguator: u8,
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
}
