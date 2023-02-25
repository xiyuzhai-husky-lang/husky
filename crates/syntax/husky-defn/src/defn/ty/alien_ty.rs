use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ExternTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: ExternTypeDecl,
}
