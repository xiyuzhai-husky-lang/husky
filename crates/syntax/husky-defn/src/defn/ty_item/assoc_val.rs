use crate::*;
use husky_expr::ExprSheet;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_sheet: ExprSheet,
    pub decl: TypeAssociatedValueDecl,
}
