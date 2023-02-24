use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMethodDecl {
    #[id]
    pub associated_item: AssociatedItem,
    pub path: Option<TypeItemPath>,
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

impl TypeMethodDecl {
    pub fn parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [RegularParameterDeclPattern]> {
        self.parameter_decl_list(db).as_ref()?.parameters()
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
