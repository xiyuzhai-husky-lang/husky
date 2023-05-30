use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitMethodFnDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl TraitMethodFnDecl {
    pub fn parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.parameter_decl_list(db).regular_parameters()
    }
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}

impl<'a> DeclParseContext<'a> {}
