use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub path: FormPath,
    pub expr_region: ExprRegion,
    pub decl: MorphismDecl,
}
