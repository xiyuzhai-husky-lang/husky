use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn type_alias_signature(db: &dyn SignatureDb, decl: TypeAliasDecl) -> TypeAliasSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    // implementation
    TypeAliasSignature::new(
        db,
        engine.finish(),
    )
}




#[salsa::tracked(jar = SignatureJar)]
pub struct TypeAliasSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}
