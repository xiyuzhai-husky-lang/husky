use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleStructTypeDefn {
    #[id]
    pub node_id: TypeNodeId,
    pub decl: TupleStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn tuple_struct_ty_defn(
    db: &dyn DefnDb,
    decl: TupleStructTypeDecl,
) -> TupleStructTypeDefn {
    let node_id = decl.node_id(db);
    TupleStructTypeDefn::new(db, node_id, decl)
}
