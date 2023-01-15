use super::*;
use husky_token::EolColonToken;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeImplBlockDecl {
    pub ast_idx: AstIdx,
    pub impl_block: ImplBlock,
    pub impl_token: ImplToken,
    pub ty: ExprIdx,
    #[return_ref]
    pub eol_colon: ExprResult<EolColonToken>,
    pub expr_sheet: ExprSheet,
}
