use crate::*;

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAssociatedTypeSignature {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
}
