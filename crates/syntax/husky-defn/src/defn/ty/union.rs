use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnionTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: UnionTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn union_ty_defn(db: &dyn DefnDb, decl: UnionTypeDecl) -> UnionTypeDefn {
    let node_id = decl.node_id(db);
    UnionTypeDefn::new(db, node_id, decl)
}
