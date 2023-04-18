use super::*;
use husky_entity_tree::TraitForTypeImplBlock;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeImplBlockDeclarativeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub trai: DeclarativeTerm,
    pub ty: DeclarativeTerm,
}

impl HasDeclarativeSignature for TraitForTypeImplBlock {
    type DeclarativeSignature = TraitForTypeImplBlockDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        self.decl(db)?.declarative_signature(db)
    }
}

impl HasDeclarativeSignature for TraitForTypeImplBlockDecl {
    type DeclarativeSignature = TraitForTypeImplBlockDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        trai_for_ty_impl_block_declarative_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_impl_block_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeImplBlockDecl,
) -> DeclarativeSignatureResult<TraitForTypeImplBlockDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
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
    Ok(TraitForTypeImplBlockDeclarativeSignature::new(
        db,
        implicit_parameters,
        trai,
        ty,
    ))
}
