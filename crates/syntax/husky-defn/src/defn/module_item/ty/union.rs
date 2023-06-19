use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: UnionTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn union_ty_defn(db: &dyn DefnDb, decl: UnionTypeDecl) -> UnionTypeDefn {
    let node_path = decl.node_path(db);
    UnionTypeDefn::new(db, node_path, decl)
}
