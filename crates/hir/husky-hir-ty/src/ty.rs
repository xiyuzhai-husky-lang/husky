pub mod path_leading;
pub mod ritchie;

use self::{path_leading::HirTypePathLeading, ritchie::HirRitchieType};
use crate::*;
use either::*;
use husky_eth_signature::{
    helpers::trai_for_ty::is_ty_term_always_copyable, signature::HasEthTemplate,
};
use husky_eth_term::term::{
    application::{EthApplication, TermFunctionReduced},
    symbolic_variable::EthTermSymbolIndexImpl,
    EthTerm,
};
use husky_fly_term::{FlyTerm, FlyTermBase, FlyTerms};
use husky_term_prelude::ItemPathTerm;

/// this is much simpler than that in Term, right?
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirType {
    PathLeading(HirTypePathLeading),
    Variable(HirTypeTemplateVariable),
    TypeAssocType(HirTypeTypeAssocType),
    TraitAssocType(HirTypeTraitAssocType),
    Ritchie(HirRitchieType),
}

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypeTypeAssocType {}

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypeTraitAssocType {}

impl HirType {
    pub fn from_eth(term: EthTerm, db: &::salsa::Db) -> Option<Self> {
        let always_copyable = is_ty_term_always_copyable(term, db).unwrap()?;
        match term {
            EthTerm::SymbolicVariable(symbol) => {
                HirTypeTemplateVariable::from_eth(symbol, db).map(Into::into)
            }
            EthTerm::EntityPath(path) => match path {
                ItemPathTerm::Form(_) => todo!(),
                ItemPathTerm::Trait(_) => todo!(),
                ItemPathTerm::TypeOntology(ty_path) => {
                    Some(HirTypePathLeading::new(db, ty_path, smallvec![], always_copyable).into())
                }
                ItemPathTerm::TypeInstance(_) => todo!(),
                ItemPathTerm::TypeVariant(_) => todo!(),
            },
            EthTerm::Ritchie(term_ritchie) => {
                Some(HirRitchieType::from_eth(term_ritchie, db).into())
            }
            EthTerm::Application(term_application) => {
                Some(hir_ty_from_eth_term_application(db, term_application))
            }
            EthTerm::TypeAsTraitItem(_) => todo!(),
            _ => unreachable!("it should be guaranteed that the term is a valid HirType"),
        }
    }

    /// this will ignore the place
    pub fn from_fly(term: FlyTerm, db: &::salsa::Db, fly_terms: &FlyTerms) -> Option<Self> {
        // todo: consider place
        match term.base_resolved_inner(fly_terms) {
            FlyTermBase::Eth(term) => HirType::from_eth(term, db),
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(_) => unreachable!("expected all fly terms to be resolved"),
            FlyTermBase::Place => todo!(),
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

    pub fn always_copyable(self, db: &::salsa::Db) -> bool {
        match self {
            HirType::PathLeading(slf) => slf.always_copyable(db),
            HirType::Variable(_slf) => false, // ad hoc: todo check traits
            HirType::TypeAssocType(_slf) => false, // ad hoc: todo check traits
            HirType::TraitAssocType(_slf) => false, // ad hoc: todo check traits
            HirType::Ritchie(_slf) => true,
        }
    }
    pub fn is_float(self, db: &::salsa::Db) -> bool {
        match self {
            HirType::PathLeading(field_ty)
                if let Left(PreludeTypePath::Num(PreludeNumTypePath::Float(_))) =
                    field_ty.ty_path(db).refine(db) =>
            {
                true
            }
            _ => false,
        }
    }
}

#[salsa::tracked(jar = HirTypeJar)]
pub(crate) fn hir_ty_from_eth_term_application(
    db: &::salsa::Db,
    term_application: EthApplication,
) -> HirType {
    let application_expansion = term_application.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => {
            let ty_eth_template = ty_path.eth_template(db).expect("ok");
            let template_parameters = ty_eth_template.template_parameters(db);
            // filter out phantoms
            let template_arguments = std::iter::zip(
                template_parameters.iter(),
                application_expansion.arguments(db).iter().copied(),
            )
            .filter_map(|(param, arg)| {
                match param.symbol().index(db).inner() {
                    EthTermSymbolIndexImpl::ExplicitLifetime { attrs, .. }
                    | EthTermSymbolIndexImpl::ExplicitPlace { attrs, .. }
                    | EthTermSymbolIndexImpl::Type { attrs, .. }
                    | EthTermSymbolIndexImpl::ConstOther { attrs, .. }
                    | EthTermSymbolIndexImpl::ConstPathLeading { attrs, .. } => !attrs.phantom(),
                    EthTermSymbolIndexImpl::Prop { .. } => false,
                    EthTermSymbolIndexImpl::EphemPathLeading { .. }
                    | EthTermSymbolIndexImpl::EphemOther { .. }
                    | EthTermSymbolIndexImpl::SelfType
                    | EthTermSymbolIndexImpl::SelfValue
                    | EthTermSymbolIndexImpl::SelfLifetime
                    | EthTermSymbolIndexImpl::SelfPlace => unreachable!(),
                }
                .then_some(arg)
            })
            .map(|arg| HirTemplateArgument::from_eth(arg, db).unwrap())
            .collect();
            HirTypePathLeading::new(
                db,
                ty_path,
                template_arguments,
                is_ty_term_always_copyable(term_application.into(), db)
                    .unwrap()
                    .expect("should be a hir ty"),
            )
            .into()
        }
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}
