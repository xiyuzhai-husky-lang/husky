use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: UnitStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn unit_struct_ty_defn(db: &dyn DefnDb, decl: UnitStructTypeDecl) -> UnitStructTypeDefn {
    todo!()
    // let node_path = decl.node_path(db);
    // UnitStructTypeDefn::new(db, node_path, decl)
}
