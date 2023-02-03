use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAsTraitMethodDecl {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub parameter_decl_list: ParameterDeclList,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}

impl TypeAsTraitMethodDecl {
    pub fn parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.parameter_decl_list(db).parameters()
    }
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}
