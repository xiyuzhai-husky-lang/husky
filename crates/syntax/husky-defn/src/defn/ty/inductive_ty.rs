use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: InductiveTypeDecl,
}
