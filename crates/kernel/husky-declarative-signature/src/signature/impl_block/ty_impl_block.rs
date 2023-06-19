use husky_entity_tree::TypeImplBlockNode;

use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeImplBlockDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    pub ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeImplBlockPath {
    type DeclarativeSignatureTemplate = TypeImplBlockDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        self.decl(db)?.declarative_signature_template(db)
    }
}

impl HasDeclarativeSignatureTemplate for TypeImplBlockDecl {
    type DeclarativeSignatureTemplate = TypeImplBlockDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_impl_block_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_impl_block_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeImplBlockDecl,
) -> DeclarativeSignatureResult<TypeImplBlockDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let ty_expr = decl.ty_expr(db);
    let ty = match declarative_term_region.expr_term(ty_expr.expr()) {
        Ok(ty) => ty,
        Err(_) => todo!(),
    };
    Ok(TypeImplBlockDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
        ty,
    ))
}
