use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: EnumTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn enum_ty_defn(db: &dyn DefnDb, decl: EnumTypeDecl) -> EnumTypeDefn {
    let node_path = decl.node_path(db);
    EnumTypeDefn::new(db, node_path, decl)
}
