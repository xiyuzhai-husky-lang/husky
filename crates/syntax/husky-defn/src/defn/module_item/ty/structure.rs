use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: StructureTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn structure_ty_defn(db: &dyn DefnDb, decl: StructureTypeDecl) -> StructureTypeDefn {
    let node_path = decl.node_path(db);
    StructureTypeDefn::new(db, node_path, decl)
}
