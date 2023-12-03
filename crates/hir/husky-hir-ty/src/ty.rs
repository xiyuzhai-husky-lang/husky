pub mod path_leading;
pub mod ritchie;

use self::{path_leading::HirTypePathLeading, ritchie::HirRitchieType};
use crate::*;
use husky_ethereal_signature::HasEtherealSignatureTemplate;
use husky_ethereal_term::{
    EtherealTerm, EtherealTermApplication, EtherealTermRitchie, EtherealTermSymbolIndexInner,
    TermFunctionReduced,
};
use husky_print_utils::p;
use husky_term_prelude::TermEntityPath;
use salsa::DebugWithDb;

/// this is much simpler than that in Term, right?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = HirTypeDb, jar = HirTypeJar)]
pub enum HirType {
    PathLeading(HirTypePathLeading),
    Symbol(HirTypeSymbol),
    TypeAssociatedType(HirTypeTypeAssociatedType),
    TraitAssociatedType(HirTypeTraitAssociatedType),
    Ritchie(HirRitchieType),
}

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypeTypeAssociatedType {}

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypeTraitAssociatedType {}

impl HirType {
    pub fn from_ethereal(term: EtherealTerm, db: &::salsa::Db) -> Option<Self> {
        match term {
            EtherealTerm::Symbol(symbol) => {
                HirTypeSymbol::from_ethereal(symbol, db).map(Into::into)
            }
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(ty_path) => {
                    Some(HirTypePathLeading::new(db, ty_path, smallvec![]).into())
                }
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Ritchie(term_ritchie) => {
                Some(HirRitchieType::from_ethereal(term_ritchie, db).into())
            }
            EtherealTerm::Application(term_application) => {
                Some(hir_ty_from_ethereal_term_application(db, term_application))
            }
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            _ => unreachable!("it should be guaranteed that the term is a valid HirType"),
        }
    }

    pub fn prelude_ty_path(self, db: &::salsa::Db) -> Option<PreludeTypePath> {
        match self {
            HirType::PathLeading(hir_ty) => hir_ty.ty_path(db).prelude_ty_path(db),
            _ => None,
        }
    }

    pub fn is_core_basic_unit_obviously(self, db: &::salsa::Db) -> bool {
        match self.prelude_ty_path(db) {
            Some(PreludeTypePath::UNIT) => true,
            _ => false,
        }
    }

    pub fn is_copyable_obviously(self, db: &::salsa::Db) -> bool {
        match self {
            HirType::PathLeading(slf) => slf.is_copyable_obviously(db),
            HirType::Symbol(slf) => false, // ad hoc: todo check traits
            HirType::TypeAssociatedType(slf) => false, // ad hoc: todo check traits
            HirType::TraitAssociatedType(slf) => false, // ad hoc: todo check traits
            HirType::Ritchie(slf) => true,
        }
    }
}

#[salsa::tracked(jar = HirTypeJar)]
pub(crate) fn hir_ty_from_ethereal_term_application(
    db: &::salsa::Db,
    term_application: EtherealTermApplication,
) -> HirType {
    let application_expansion = term_application.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => {
            let ty_ethereal_signature_template =
                ty_path.ethereal_signature_template(db).expect("ok");
            let template_parameters = ty_ethereal_signature_template.template_parameters(db);
            // filter out phantoms
            let template_arguments = std::iter::zip(
                template_parameters.iter(),
                application_expansion.arguments(db).iter().copied(),
            )
            .filter_map(|(param, arg)| {
                match param.symbol().index(db).inner() {
                    EtherealTermSymbolIndexInner::ExplicitLifetime { attrs, .. }
                    | EtherealTermSymbolIndexInner::ExplicitPlace { attrs, .. }
                    | EtherealTermSymbolIndexInner::Type { attrs, .. }
                    | EtherealTermSymbolIndexInner::ConstOther { attrs, .. } => !attrs.phantom(),
                    EtherealTermSymbolIndexInner::Prop { .. } => false,
                    EtherealTermSymbolIndexInner::ConstPathLeading { attrs, .. } => {
                        !attrs.phantom()
                    }
                    EtherealTermSymbolIndexInner::EphemPathLeading { .. }
                    | EtherealTermSymbolIndexInner::EphemOther { .. }
                    | EtherealTermSymbolIndexInner::SelfType
                    | EtherealTermSymbolIndexInner::SelfValue
                    | EtherealTermSymbolIndexInner::SelfLifetime
                    | EtherealTermSymbolIndexInner::SelfPlace => unreachable!(),
                }
                .then_some(arg)
            })
            .map(|arg| HirTemplateArgument::from_ethereal(arg, db).unwrap())
            .collect();
            HirTypePathLeading::new(db, ty_path, template_arguments).into()
        }
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}
