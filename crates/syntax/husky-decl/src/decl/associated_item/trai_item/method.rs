use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitMethodDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    parameter_decl_list: ExplicitParameterDeclList,
    #[return_ref]
    pub curry_token: DeclExprResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclExprResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
}

impl TraitMethodDecl {
    pub fn parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ExplicitParameterDeclPattern]> {
        self.parameter_decl_list(db).parameters()
    }
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self.implicit_parameter_decl_list(db).as_ref()? {
            Some(list) => list.implicit_parameters(),
            None => Ok(&[]),
        }
    }
}

impl<'a> DeclParseContext<'a> {}
