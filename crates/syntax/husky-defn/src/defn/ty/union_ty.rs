use super::*;

#[salsa::tracked(jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeDecl,
    pub expr_sheet: ExprSheet,
}
