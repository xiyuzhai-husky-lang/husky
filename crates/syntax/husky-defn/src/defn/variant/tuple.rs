use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: TupleVariantDecl,
}
