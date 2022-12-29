use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct InductiveTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
