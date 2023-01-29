use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct AlienTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: AlienTypeDecl,
}
