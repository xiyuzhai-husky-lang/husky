use super::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TraitForTypeImplBlockDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub trai: DecTerm,
    pub self_ty: DeclarativeSelfType,
    // todo: where clause
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeclarativeSelfType {
    Path(DecTerm),
    DerivedAny(DecSvar),
}

impl DeclarativeSelfType {
    pub fn term(self) -> DecTerm {
        match self {
            DeclarativeSelfType::Path(term) => term,
            DeclarativeSelfType::DerivedAny(term) => term.into(),
        }
    }
}

impl HasDecTemplate for TraitForTypeImplBlockPath {
    type DecTemplate = TraitForTypeImplBlockDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        trai_for_ty_impl_block_syn_dec_template(db, self)
    }
}

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn trai_for_ty_impl_block_syn_dec_template(
    db: &::salsa::Db,
    path: TraitForTypeImplBlockPath,
) -> DecSignatureResult<TraitForTypeImplBlockDecTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
    let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
    let template_parameters = DecTemplateParameters::from_decl(
        decl.template_parameters(db),
        &dec_term_region,
        dec_term_menu,
    );
    let trai_expr = decl.trai_expr(db);
    let trai = dec_term_region.expr_term(trai_expr.syn_expr_idx())?;
    let self_ty_term = dec_term_region
        .dec_symbol_region()
        .self_ty()
        .ok_or(DecSignatureError::SelfTypeNotInferred)?;
    let self_ty = match decl.self_ty_decl(db) {
        SelfTypeDecl::PathLeadingExpr(ty_expr) => {
            debug_assert_eq!(
                self_ty_term,
                dec_term_region.expr_term(ty_expr.expr()).expect("ok")
            );
            DeclarativeSelfType::Path(self_ty_term)
        }
        SelfTypeDecl::DeriveAny { .. } => {
            let DecTerm::Symbol(self_ty_symbol) = self_ty_term else {
                unreachable!()
            };
            DeclarativeSelfType::DerivedAny(self_ty_symbol)
        }
    };
    Ok(TraitForTypeImplBlockDecTemplate::new(
        db,
        template_parameters,
        trai,
        self_ty,
    ))
}
