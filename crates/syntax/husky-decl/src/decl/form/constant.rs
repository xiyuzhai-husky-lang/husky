use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct ConstantDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
}
