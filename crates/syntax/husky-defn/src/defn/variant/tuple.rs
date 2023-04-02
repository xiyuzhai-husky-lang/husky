use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TupleVariantDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: TupleVariantDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn tuple_variant_defn(_db: &dyn DefnDb, _decl: TupleVariantDecl) -> TupleVariantDefn {
    todo!()
}
