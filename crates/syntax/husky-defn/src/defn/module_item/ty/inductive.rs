use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: InductiveTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn inductive_ty_defn(db: &dyn DefnDb, decl: InductiveTypeDecl) -> InductiveTypeDefn {
    let node_path = decl.node_path(db);
    InductiveTypeDefn::new(db, node_path, decl)
}
