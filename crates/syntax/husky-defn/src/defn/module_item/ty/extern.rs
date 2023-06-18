use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ExternTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: ExternTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn alien_ty_defn(db: &dyn DefnDb, decl: ExternTypeDecl) -> ExternTypeDefn {
    let node_path = decl.node_path(db);
    ExternTypeDefn::new(db, node_path, decl)
}
