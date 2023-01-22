use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAsTraitAssociatedFunctionDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub output_ty: DeclResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}
