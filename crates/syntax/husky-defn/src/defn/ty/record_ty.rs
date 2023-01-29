use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RecordTypeDecl,
}
