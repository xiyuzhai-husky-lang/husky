use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RegularStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RegularStructTypeDecl,
}
