use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeImplBlockDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub trai: DeclarativeTerm,
    pub self_ty: DeclarativeSelfType,
    // todo: where clause
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeSelfType {
    Path(DeclarativeTerm),
    DerivedAny(DeclarativeTermSymbol),
}

impl DeclarativeSelfType {
    pub fn term(self) -> DeclarativeTerm {
        match self {
            DeclarativeSelfType::Path(term) => term,
            DeclarativeSelfType::DerivedAny(term) => term.into(),
        }
    }
}

impl HasDeclarativeSignatureTemplate for TraitForTypeImplBlockPath {
    type DeclarativeSignatureTemplate = TraitForTypeImplBlockDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_for_ty_impl_block_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_impl_block_syn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TraitForTypeImplBlockPath,
) -> DeclarativeSignatureResult<TraitForTypeImplBlockDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let trai_expr = decl.trai_expr(db);
    let trai = match declarative_term_region.expr_term(trai_expr.expr()) {
        Ok(trai) => trai,
        Err(_) => todo!(),
    };
    let self_ty_term = declarative_term_region
        .term_symbol_region()
        .self_ty_term()
        .ok_or(DeclarativeSignatureError::SelfTypeNotInferred)?;
    let self_ty = match decl.self_ty_decl(db) {
        SelfTypeDecl::PathLeadingExpr(ty_expr) => {
            debug_assert_eq!(
                self_ty_term,
                declarative_term_region
                    .expr_term(ty_expr.expr())
                    .expect("ok")
            );
            DeclarativeSelfType::Path(self_ty_term)
        }
        SelfTypeDecl::DeriveAny { .. } => {
            let DeclarativeTerm::Symbol(self_ty_symbol) = self_ty_term else {
                unreachable!()
            };
            DeclarativeSelfType::DerivedAny(self_ty_symbol)
        }
    };
    Ok(TraitForTypeImplBlockDeclarativeSignatureTemplate::new(
        db,
        template_parameters,
        trai,
        self_ty,
    ))
}
