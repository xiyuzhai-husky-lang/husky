use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: StructureTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn structure_ty_defn(db: &dyn DefnDb, decl: StructureTypeDecl) -> StructureTypeDefn {
    let node_id = decl.node_id(db);
    StructureTypeDefn::new(db, node_id, decl)
}
