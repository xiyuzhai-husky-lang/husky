use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: UnitStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn unit_struct_ty_defn(db: &dyn DefnDb, decl: UnitStructTypeDecl) -> UnitStructTypeDefn {
    let node_id = decl.node_id(db);
    UnitStructTypeDefn::new(db, node_id, decl)
}
