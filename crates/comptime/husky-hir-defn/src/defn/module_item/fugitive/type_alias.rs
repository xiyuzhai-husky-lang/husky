use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct TypeAliasHirDefn {
    #[id]
    pub path: FugitivePath,
    pub expr_region: HirExprRegion,
}
