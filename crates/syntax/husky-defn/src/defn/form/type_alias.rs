use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAliasDefn {
    #[id]
    pub path: FormPath,
    pub decl: TypeAliasDecl,
    pub expr_region: ExprRegion,
}
