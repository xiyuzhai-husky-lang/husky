use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
}
