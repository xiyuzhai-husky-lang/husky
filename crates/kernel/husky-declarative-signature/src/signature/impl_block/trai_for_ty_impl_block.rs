use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeImplBlockDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameters,
    pub trai: DeclarativeTerm,
    pub self_ty: DeclarativeSelfType,
    // todo: where clause
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeSelfType {
    Path(DeclarativeTerm),
    Parameter(DeclarativeTermSymbol),
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
    let generic_parameters = DeclarativeGenericParameters::from_decl(
        decl.generic_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let trai_expr = decl.trai_expr(db);
    let trai = match declarative_term_region.expr_term(trai_expr.expr()) {
        Ok(trai) => trai,
        Err(_) => todo!(),
    };
    // let ty_expr =
    match decl.self_ty_decl(db) {
        SelfTypeDecl::Expr(_) => todo!(),
        SelfTypeDecl::DeriveAny {} => todo!(),
    };
    // let ty = match declarative_term_region.expr_term(ty_expr.expr()) {
    //     Ok(ty) => ty,
    //     Err(_) => todo!(),
    // };
    // Ok(TraitForTypeImplBlockDeclarativeSignatureTemplate::new(
    //     db,
    //     generic_parameters,
    //     trai,
    //     ty,
    // ))
}
