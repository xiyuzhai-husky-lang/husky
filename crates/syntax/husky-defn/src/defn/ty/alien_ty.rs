use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ExternTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: ExternTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn alien_ty_defn(db: &dyn DefnDb, decl: ExternTypeDecl) -> ExternTypeDefn {
    let path = decl.path(db);
    ExternTypeDefn::new(db, path, decl)
}
