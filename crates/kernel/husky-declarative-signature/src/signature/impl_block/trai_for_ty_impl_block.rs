use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeImplBlockDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    pub trai: DeclarativeTerm,
    pub ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TraitForTypeImplBlockPath {
    type DeclarativeSignatureTemplate = TraitForTypeImplBlockDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_for_ty_impl_block_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_impl_block_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TraitForTypeImplBlockPath,
) -> DeclarativeSignatureResult<TraitForTypeImplBlockDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let trai_expr = decl.trai_expr(db);
    let trai = match declarative_term_region.expr_term(trai_expr.expr()) {
        Ok(trai) => trai,
        Err(_) => todo!(),
    };
    let ty_expr = decl.ty_expr(db);
    let ty = match declarative_term_region.expr_term(ty_expr.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TraitForTypeImplBlockDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
        trai,
        ty,
    ))
}
