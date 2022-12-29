use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct RecordTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
}
