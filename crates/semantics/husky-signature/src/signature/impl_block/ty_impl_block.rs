use husky_entity_tree::TypeImplBlock;

use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeImplBlockSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    pub ty: DeclarativeTerm,
}

impl HasDeclarativeSignature for TypeImplBlock {
    type DeclarativeSignature = TypeImplBlockSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        self.decl(db)?.declarative_signature(db)
    }
}

impl HasDeclarativeSignature for TypeImplBlockDecl {
    type DeclarativeSignature = TypeImplBlockSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        ty_impl_block_declarative_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_impl_block_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeImplBlockDecl,
) -> DeclarativeSignatureResult<TypeImplBlockSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let ty_expr = decl.ty_expr(db);
    let ty = match declarative_term_region.expr_term(ty_expr.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TypeImplBlockSignature::new(db, implicit_parameters, ty))
}
