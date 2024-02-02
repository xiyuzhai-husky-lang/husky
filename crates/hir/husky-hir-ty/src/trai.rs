use crate::*;
use husky_ethereal_signature::HasEthTemplate;
use husky_ethereal_term::{
    ApplicationEtherealTerm, EtherealTerm, EtherealTermSymbolIndexImpl, TermFunctionReduced,
};
use husky_term_prelude::ItemPathTerm;

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTrait {
    pub trai_path: TraitPath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: HirTemplateArguments,
}

impl HirTrait {
    pub fn from_ethereal(trai_term: EtherealTerm, db: &::salsa::Db) -> Self {
        match trai_term {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(_) => todo!(),
            EtherealTerm::Rune(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                ItemPathTerm::Fugitive(_) => todo!(),
                ItemPathTerm::Trait(trai_path) => Self::new(db, trai_path, smallvec![]),
                ItemPathTerm::TypeOntology(_) => todo!(),
                ItemPathTerm::TypeInstance(_) => todo!(),
                ItemPathTerm::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(trai_term) => {
                hir_trai_from_ethereal_term_application(db, trai_term)
            }
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_trai_from_ethereal_term_application(
    db: &::salsa::Db,
    trai_term: ApplicationEtherealTerm,
) -> HirTrait {
    let application_expansion = trai_term.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(_) => todo!(),
        TermFunctionReduced::Trait(trai_path) => {
            let ty_ethereal_signature_template =
                trai_path.ethereal_signature_template(db).expect("ok");
            // todo: turn the following into utils
            let template_parameters = ty_ethereal_signature_template.template_parameters(db);
            // filter out phantoms
            let template_arguments = std::iter::zip(
                template_parameters.iter(),
                application_expansion.arguments(db).iter().copied(),
            )
            .filter_map(|(param, arg)| match param.symbol().index(db).inner() {
                EtherealTermSymbolIndexImpl::ExplicitLifetime {
                    attrs: _,
                    variance: _,
                    disambiguator: _,
                } => todo!(),
                EtherealTermSymbolIndexImpl::ExplicitPlace {
                    attrs: _,
                    variance: _,
                    disambiguator: _,
                } => todo!(),
                EtherealTermSymbolIndexImpl::Type { attrs, .. } => (!attrs.phantom())
                    .then(|| HirTemplateArgument::Type(HirType::from_ethereal(arg, db).unwrap())),
                EtherealTermSymbolIndexImpl::Prop { .. } => None,
                EtherealTermSymbolIndexImpl::ConstPathLeading { .. }
                | EtherealTermSymbolIndexImpl::ConstOther { .. } => todo!(),
                EtherealTermSymbolIndexImpl::EphemPathLeading { .. }
                | EtherealTermSymbolIndexImpl::EphemOther { .. }
                | EtherealTermSymbolIndexImpl::SelfType
                | EtherealTermSymbolIndexImpl::SelfValue
                | EtherealTermSymbolIndexImpl::SelfLifetime
                | EtherealTermSymbolIndexImpl::SelfPlace => unreachable!(),
            })
            .collect();
            HirTrait::new(db, trai_path, template_arguments).into()
        }
        TermFunctionReduced::Other(_) => todo!(),
    }
}
