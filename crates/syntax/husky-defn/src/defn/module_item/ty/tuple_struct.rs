use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleStructTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: TupleStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn tuple_struct_ty_defn(
    db: &dyn DefnDb,
    decl: TupleStructTypeDecl,
) -> TupleStructTypeDefn {
    let node_path = decl.node_path(db);
    TupleStructTypeDefn::new(db, node_path, decl)
}
