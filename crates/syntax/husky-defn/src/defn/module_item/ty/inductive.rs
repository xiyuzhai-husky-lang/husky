use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: InductiveTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn inductive_ty_defn(db: &dyn DefnDb, decl: InductiveTypeDecl) -> InductiveTypeDefn {
    let node_id = decl.node_id(db);
    InductiveTypeDefn::new(db, node_id, decl)
}
