use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn unit_struct_ty_defn(db: &dyn DefnDb, decl: UnitStructTypeDecl) -> UnitStructTypeDefn {
    let path = decl.path(db);
    UnitStructTypeDefn::new(db, path, decl)
}
