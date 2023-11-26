use crate::*;
use husky_ethereal_signature::HasEtherealSignatureTemplate;
use husky_ethereal_term::{
    EtherealTerm, EtherealTermApplication, EtherealTermSymbolIndexInner, TermFunctionReduced,
};
use husky_term_prelude::TermEntityPath;

#[salsa::interned(db = HirTypeDb, jar = HirTypeJar)]
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
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(trai_path) => Self::new(db, trai_path, smallvec![]),
                TermEntityPath::TypeOntology(_) => todo!(),
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(trai_term) => {
                hir_trai_from_ethereal_term_application(db, trai_term)
            }
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_trai_from_ethereal_term_application(
    db: &::salsa::Db,
    trai_term: EtherealTermApplication,
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
                EtherealTermSymbolIndexInner::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => todo!(),
                EtherealTermSymbolIndexInner::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator,
                } => todo!(),
                EtherealTermSymbolIndexInner::Type { attrs, .. } => (!attrs.phantom())
                    .then(|| HirTemplateArgument::Type(HirType::from_ethereal(arg, db))),
                EtherealTermSymbolIndexInner::Prop { .. } => None,
                EtherealTermSymbolIndexInner::ConstPathLeading { .. }
                | EtherealTermSymbolIndexInner::ConstOther { .. } => todo!(),
                EtherealTermSymbolIndexInner::EphemPathLeading { .. }
                | EtherealTermSymbolIndexInner::EphemOther { .. }
                | EtherealTermSymbolIndexInner::SelfType
                | EtherealTermSymbolIndexInner::SelfValue
                | EtherealTermSymbolIndexInner::SelfLifetime
                | EtherealTermSymbolIndexInner::SelfPlace => unreachable!(),
            })
            .collect();
            HirTrait::new(db, trai_path, template_arguments).into()
        }
        TermFunctionReduced::Other(_) => todo!(),
    }
}
