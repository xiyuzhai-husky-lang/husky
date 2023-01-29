use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ValueDefn {
    #[id]
    pub path: FormPath,
    pub expr_region: ExprRegion,
    pub decl: ValueDecl,
}
