use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RecordTypeDecl,
}
