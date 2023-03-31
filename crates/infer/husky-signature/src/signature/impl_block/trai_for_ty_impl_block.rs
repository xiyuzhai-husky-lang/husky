use super::*;
use husky_entity_tree::TraitForTypeImplBlock;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitForTypeImplBlockSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub trai: RawTerm,
    pub ty: RawTerm,
}

impl HasSignature for TraitForTypeImplBlock {
    type Signature = TraitForTypeImplBlockSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        trai_for_ty_impl_block_signature(db, self.decl(db)?)
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_for_ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TraitForTypeImplBlockDecl,
) -> SignatureResult<TraitForTypeImplBlockSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &signature_term_region,
        raw_term_menu,
    );
    let trai_expr = decl.trai_expr(db);
    let trai = match signature_term_region.expr_term(trai_expr.expr()) {
        Ok(trai) => trai,
        Err(_) => todo!(),
    };
    let ty_expr = decl.ty_expr(db);
    let ty = match signature_term_region.expr_term(ty_expr.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TraitForTypeImplBlockSignature::new(
        db,
        implicit_parameters,
        trai,
        ty,
    ))
}
