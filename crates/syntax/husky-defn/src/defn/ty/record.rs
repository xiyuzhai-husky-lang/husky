use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RecordTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn record_ty_defn(db: &dyn DefnDb, decl: RecordTypeDecl) -> RecordTypeDefn {
    let path = decl.path(db);
    RecordTypeDefn::new(db, path, decl)
}
