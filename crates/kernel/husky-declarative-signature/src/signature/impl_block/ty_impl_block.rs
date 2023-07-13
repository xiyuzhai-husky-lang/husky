use husky_entity_tree::{TypeImplBlockNode, TypeImplBlockNodePath};

use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeImplBlockDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameters,
    pub ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeImplBlockPath {
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
    path: TypeImplBlockPath,
) -> DeclarativeSignatureResult<TypeImplBlockDeclarativeSignatureTemplate> {
    let decl = path.decl(db)?;
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let generic_parameters = DeclarativeGenericParameters::from_decl(
        decl.generic_parameters(db),
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
        generic_parameters,
        ty,
    ))
}
