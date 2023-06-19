use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: EnumTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn enum_ty_defn(db: &dyn DefnDb, decl: EnumTypeDecl) -> EnumTypeDefn {
    let node_id = decl.node_id(db);
    EnumTypeDefn::new(db, node_id, decl)
}
