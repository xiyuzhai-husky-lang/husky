use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnitStructTypeHirDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeHirDecl,
}
