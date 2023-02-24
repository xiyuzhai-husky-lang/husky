use husky_token::{CurryToken, EolColonToken};

use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FunctionDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    parameter_decl_list: DeclExprResult<RegularParameterDeclList>,
    #[return_ref]
    pub curry_token: DeclExprResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclExprResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
}

impl FunctionDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        self.implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(Ok(&[]))
    }

    pub fn parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [RegularParameterDeclPattern]> {
        self.parameter_decl_list(db).as_ref()?.parameters()
    }
}
