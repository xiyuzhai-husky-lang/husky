use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct PropsStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: PropsStructTypeDecl,
    pub expr_sheet: ExprSheet,
}
