use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct TypeAliasHirDefn {
    #[id]
    pub path: FugitivePath,
    pub hir_expr_region: HirExprRegion,
}
