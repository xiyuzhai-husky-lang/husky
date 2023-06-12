use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: RecordTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn record_ty_defn(db: &dyn DefnDb, decl: RecordTypeDecl) -> RecordTypeDefn {
    let node_path = decl.node_path(db);
    RecordTypeDefn::new(db, node_path, decl)
}
