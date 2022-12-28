use super::*;

#[salsa::tracked(jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
