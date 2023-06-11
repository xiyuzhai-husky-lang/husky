use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RegularStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: RegularStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn regular_struct_ty_defn(
    db: &dyn DefnDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeDefn {
    let path = decl.path(db);
    RegularStructTypeDefn::new(db, path, decl)
}
