use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeDecl,
}
