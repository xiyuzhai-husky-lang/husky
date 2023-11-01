use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnitStructTypeHirDefn {
    pub path: TypePath,
    pub hir_decl: UnitStructTypeHirDecl,
}
