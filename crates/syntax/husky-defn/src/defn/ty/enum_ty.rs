use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
}
