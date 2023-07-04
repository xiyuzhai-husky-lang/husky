use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitMethodFnNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    parameter_decl_list: SelfParameterAndExplicitParameters<true>,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitMethodFnDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub self_parameter: Option<SelfParameterDeclPattern>,
    #[return_ref]
    pub explicit_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl TraitMethodFnDecl {}
