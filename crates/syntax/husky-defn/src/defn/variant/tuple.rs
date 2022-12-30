use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TupleVariantDefn {
    #[id]
    pub path: VariantPath,
    pub decl: TupleVariantDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
