use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct RecordTypeDecl {
    pub path: TypePath,
}
