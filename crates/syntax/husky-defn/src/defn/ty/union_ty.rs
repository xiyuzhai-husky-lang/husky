use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnionTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn union_ty_defn(db: &dyn DefnDb, decl: UnionTypeDecl) -> UnionTypeDefn {
    let path = decl.path(db);
    UnionTypeDefn::new(db, path, decl)
}
