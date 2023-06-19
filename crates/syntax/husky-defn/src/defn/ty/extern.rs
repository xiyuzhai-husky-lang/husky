use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ExternTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: ExternTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn alien_ty_defn(db: &dyn DefnDb, decl: ExternTypeDecl) -> ExternTypeDefn {
    let node_id = decl.node_id(db);
    ExternTypeDefn::new(db, node_id, decl)
}
