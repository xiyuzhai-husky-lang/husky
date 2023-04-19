use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn enum_ty_defn(db: &dyn DefnDb, decl: EnumTypeDecl) -> EnumTypeDefn {
    let path = decl.path(db);
    EnumTypeDefn::new(db, path, decl)
}
