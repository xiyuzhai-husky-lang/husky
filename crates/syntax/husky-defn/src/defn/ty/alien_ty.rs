use super::*;

#[salsa::tracked(jar = DefnJar)]
pub struct AlienTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: AlienTypeDecl,
    pub expr_sheet: ExprSheet,
}
