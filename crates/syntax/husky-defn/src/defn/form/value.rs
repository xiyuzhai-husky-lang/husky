use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct ValueDefn {
    #[id]
    pub path: FormPath,
    pub expr_page: ExprPage,
    pub decl: ValueDecl,
}
