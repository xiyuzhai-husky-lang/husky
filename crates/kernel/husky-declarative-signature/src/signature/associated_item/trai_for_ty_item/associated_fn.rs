use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub(crate) fn trai_for_ty_associated_fn_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeAssociatedFunctionDecl,
) -> DeclarativeSignatureResult<TraitForTypeAssociatedFnDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitForTypeAssociatedFnDeclarativeSignatureTemplate::new(
        db,
        todo!(),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct TraitForTypeAssociatedFnDeclarativeSignatureTemplate {
    pub return_ty: DeclarativeTerm,
}
