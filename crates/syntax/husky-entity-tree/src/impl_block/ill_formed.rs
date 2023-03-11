use super::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct IllFormedImplBlock {
    #[id]
    pub id: TypeImplId,
    pub ast_idx: AstIdx,
    pub body: AstIdxRange,
    #[return_ref]
    pub ill_form: ImplBlockIllForm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct IllFormedImplId {
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
