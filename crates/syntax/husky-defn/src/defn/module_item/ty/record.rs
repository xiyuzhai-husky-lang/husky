use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: RecordTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn record_ty_defn(db: &dyn DefnDb, decl: RecordTypeDecl) -> RecordTypeDefn {
    let node_id = decl.node_id(db);
    RecordTypeDefn::new(db, node_id, decl)
}
