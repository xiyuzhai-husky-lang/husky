use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct ValueDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
}
