use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_associated_form_fn_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeAssociatedFunctionDecl,
) -> DeclarativeSignatureResult<TraitForTypeAssociatedFnSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitForTypeAssociatedFnSignature::new(db, todo!()))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeAssociatedFnSignature {
    pub return_ty: DeclarativeTerm,
}
