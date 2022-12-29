use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FunctionDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
}
