use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct AlienTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
