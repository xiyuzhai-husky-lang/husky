use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeMethodDecl {
    #[id]
    pub associated_item: AssociatedItem,
    pub path: Option<TypeItemPath>,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub parameter_decl_list: ParameterDeclList,
    #[return_ref]
    pub curry_token: DeclResult<CurryToken>,
    #[return_ref]
    pub output_ty: DeclResult<ExprIdx>,
    #[return_ref]
    pub eol_colon: DeclResult<EolColonToken>,
}

impl TypeMethodDecl {
    pub fn parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ParameterDecl] {
        self.parameter_decl_list(db).parameters()
    }
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}
