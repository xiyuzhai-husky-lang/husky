use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct UnionTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
