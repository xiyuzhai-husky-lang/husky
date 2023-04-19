use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct GnDefn {
    #[id]
    pub path: FugitivePath,
    pub expr_region: ExprRegion,
    pub decl: GnDecl,
}
