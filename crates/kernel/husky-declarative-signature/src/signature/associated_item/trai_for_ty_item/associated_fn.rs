use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_associated_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeAssociatedFnSynDecl,
) -> DeclarativeSignatureResult<TraitForTypeAssociatedFnDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let _declarative_term_region = declarative_term_region(db, syn_expr_region);
    let _declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    Ok(TraitForTypeAssociatedFnDeclarativeSignatureTemplate::new(
        db,
        todo!(),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeAssociatedFnDeclarativeSignatureTemplate {
    pub return_ty: DeclarativeTerm,
}
