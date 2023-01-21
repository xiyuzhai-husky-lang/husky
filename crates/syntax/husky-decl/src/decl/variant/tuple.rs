use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TupleVariantDecl {
    #[id]
    pub path: VariantPath,
    pub expr_region: ExprRegion,
}
