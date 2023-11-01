use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TypeAliasHirDefn {
    pub path: FugitivePath,
    pub hir_expr_region: HirEagerExprRegion,
}
