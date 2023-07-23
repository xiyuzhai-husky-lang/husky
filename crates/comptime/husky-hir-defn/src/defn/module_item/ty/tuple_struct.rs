use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct TupleStructTypeHirDefn {
    #[id]
    pub path: TypePath,
    pub hir_decl: TupleStructTypeHirDecl,
}
