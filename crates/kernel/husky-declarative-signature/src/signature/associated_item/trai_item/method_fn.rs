use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub(crate) fn trai_method_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitMethodFnDecl,
) -> DeclarativeSignatureResult<TraitMethodSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitMethodSignature::new(db, todo!()))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct TraitMethodSignature {
    pub return_ty: DeclarativeTerm,
}
