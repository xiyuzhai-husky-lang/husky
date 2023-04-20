use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct TraitForTypeImplTemplate {
    template_parameters: TemplateParameters,
    trai_path: TraitPath,
    ty_path: Option<TypePath>,
    trai: TemplateTerm,
    ty: TemplateTerm,
    src: TraitForTypeImplTemplateSource,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub enum TraitForTypeImplTemplateSource {
    ImplBlock(TraitForTypeImplBlock),
    DeriveDecr,
}

impl TraitForTypeImplTemplate {
    pub(crate) fn collect_from_decr<'a>(
        db: &'a dyn EtherealTermDb,
        path: TypePath,
        decr: Decr,
        cards: &mut Vec<Self>,
    ) -> TermResult<()> {
        let template_parameters = path.template_parameters(db)?;
        let template_parameters_data = template_parameters.data(db);
        match decr {
            Decr::Derive(derive_decr) => {
                for trai in derive_decr.declarative_signature_template(db)?.traits(db) {
                    let trai = EtherealTerm::from_declarative(db, trai, TermTypeExpectation::Any)?;
                    cards.push(TraitForTypeImplTemplate {
                        template_parameters,
                        trai_path: trai
                            .leading_trai_path(db)
                            .ok_or(TermError::ExpectTraitForDeriveArgument)?,
                        ty_path: Some(path),
                        trai: TemplateTerm::new(db, trai, template_parameters_data),
                        ty: TemplateTerm::self_ty(db, path, template_parameters_data),
                        src: TraitForTypeImplTemplateSource::DeriveDecr,
                    })
                }
                Ok(())
            }
        }
    }

    pub fn trai_path(&self) -> TraitPath {
        self.trai_path
    }

    pub fn ty_path(&self) -> Option<TypePath> {
        self.ty_path
    }

    pub fn trai(&self) -> EtherealTerm {
        self.trai.term()
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty.term()
    }

    pub fn src(&self) -> TraitForTypeImplTemplateSource {
        self.src
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn trai_for_type_impl_template_from_impl_block(
    db: &dyn EtherealTermDb,
    impl_block: TraitForTypeImplBlock,
) -> TermResult<TraitForTypeImplTemplate> {
    let signature = impl_block.declarative_signature_template(db)?;
    let template_parameters = signature.template_parameters(db)?;
    let template_parameters_data = template_parameters.data(db);
    let trai = EtherealTerm::from_declarative(db, signature.trai(db), TermTypeExpectation::Any)?;
    let ty = EtherealTerm::ty_from_declarative(db, signature.ty(db))?;
    Ok(TraitForTypeImplTemplate {
        template_parameters,
        trai_path: trai.leading_trai_path(db).expect("should be valid trait"),
        ty_path: ty.leading_ty_path(db),
        trai: TemplateTerm::new(db, trai, template_parameters_data),
        ty: TemplateTerm::new(db, ty, template_parameters_data),
        src: TraitForTypeImplTemplateSource::ImplBlock(impl_block),
    })
}

impl HasTemplate for TraitForTypeImplBlock {
    type Template = TraitForTypeImplTemplate;

    fn template(self, db: &dyn EtherealTermDb) -> TermResult<Self::Template> {
        trai_for_type_impl_template_from_impl_block(db, self)
    }
}
