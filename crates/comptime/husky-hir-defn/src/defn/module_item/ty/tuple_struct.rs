use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TupleStructTypeHirDefn {
    pub path: TypePath,
    pub hir_decl: TupleStructTypeHirDecl,
}
