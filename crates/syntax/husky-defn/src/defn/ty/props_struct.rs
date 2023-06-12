use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct RegularStructTypeDefn {
    #[id]
    pub node_path: TypeNodePath,
    pub decl: RegularStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn regular_struct_ty_defn(
    db: &dyn DefnDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeDefn {
    let node_path = decl.node_path(db);
    RegularStructTypeDefn::new(db, node_path, decl)
}
