use crate::*;
use husky_expr::ExprSheet;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAssociatedTypeDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_sheet: ExprSheet,
    pub decl: TypeAssociatedTypeDecl,
}
