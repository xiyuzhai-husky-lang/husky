use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct EnumTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
