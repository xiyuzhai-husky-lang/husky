use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeImplBlockDecl {
    pub ast_idx: AstIdx,
    pub impl_block: ImplBlock,
    pub impl_token: ImplToken,
    pub expr_sheet: ExprSheet,
}
