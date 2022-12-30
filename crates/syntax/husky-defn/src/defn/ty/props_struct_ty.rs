use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct PropsStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: PropsStructTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
