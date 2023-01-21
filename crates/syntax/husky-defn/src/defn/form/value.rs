use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct ValueDefn {
    #[id]
    pub path: FormPath,
    pub expr_region: ExprRegion,
    pub decl: ValueDecl,
}
