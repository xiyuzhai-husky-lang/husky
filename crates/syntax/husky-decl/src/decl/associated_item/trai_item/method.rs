use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TraitMethodDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_page: ExprPage,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub output_ty: DeclResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}
