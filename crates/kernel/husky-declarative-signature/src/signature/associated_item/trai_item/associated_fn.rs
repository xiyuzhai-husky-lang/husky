use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_associated_form_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitAssociatedFnDecl,
) -> DeclarativeSignatureResult<TraitAssociatedFnDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitAssociatedFnDeclarativeSignatureTemplate::new(
        db,
        todo!(),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitAssociatedFnDeclarativeSignatureTemplate {
    pub return_ty: DeclarativeTerm,
}
