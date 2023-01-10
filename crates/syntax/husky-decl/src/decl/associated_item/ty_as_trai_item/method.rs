use crate::*;
use husky_entity_tree::AssociatedItem;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAsTraitMethodDecl {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub associated_item: AssociatedItem,
    pub ast_idx: AstIdx,
    pub expr_sheet: ExprSheet,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub parameter_decl_list: ParameterDeclList,
}
