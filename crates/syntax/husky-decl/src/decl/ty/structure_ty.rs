use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct StructureTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
