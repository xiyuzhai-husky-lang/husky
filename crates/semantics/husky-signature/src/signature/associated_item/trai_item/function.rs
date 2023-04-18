use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_associated_form_fn_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitAssociatedFunctionDecl,
) -> DeclarativeSignatureResult<TraitAssociatedFormFnSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedFormFnSignature::new(db, todo!()))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitAssociatedFormFnSignature {
    pub return_ty: DeclarativeTerm,
}
