use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn type_alias_signature(db: &dyn SignatureDb, decl: TypeAliasDecl) -> TypeAliasSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    TypeAliasSignature::new(db)
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAliasSignature {}
