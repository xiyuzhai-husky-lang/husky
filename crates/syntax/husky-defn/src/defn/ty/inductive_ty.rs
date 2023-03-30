use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: InductiveTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn inductive_ty_defn(db: &dyn DefnDb, decl: InductiveTypeDecl) -> InductiveTypeDefn {
    let path = decl.path(db);
    InductiveTypeDefn::new(db, path, decl)
}
