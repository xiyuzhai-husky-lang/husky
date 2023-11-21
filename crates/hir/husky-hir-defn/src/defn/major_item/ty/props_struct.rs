use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct PropsStructTypeHirDefn {
    pub path: TypePath,
    pub hir_decl: PropsStructTypeHirDecl,
}
