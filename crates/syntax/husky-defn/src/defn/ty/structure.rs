use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn structure_ty_defn(db: &dyn DefnDb, decl: StructureTypeDecl) -> StructureTypeDefn {
    let path = decl.path(db);
    StructureTypeDefn::new(db, path, decl)
}
