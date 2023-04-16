use husky_entity_tree::TypeImplBlock;

use super::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeImplBlockSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub ty: DeclarativeTerm,
}

impl HasSignature for TypeImplBlock {
    type Signature = TypeImplBlockSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        self.decl(db)?.signature(db)
    }
}

impl HasSignature for TypeImplBlockDecl {
    type Signature = TypeImplBlockSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        ty_impl_block_signature(db, self)
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeImplBlockDecl,
) -> SignatureResult<TypeImplBlockSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        raw_term_menu,
    );
    let ty_expr = decl.ty_expr(db);
    let ty = match signature_term_region.expr_term(ty_expr.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TypeImplBlockSignature::new(db, implicit_parameters, ty))
}
