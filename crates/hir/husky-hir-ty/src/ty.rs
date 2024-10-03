pub mod path_leading;
pub mod ritchie;

use self::{path_leading::HirTypePathLeading, ritchie::HirRitchieType};
use crate::*;
use either::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::major_item::ty::{PreludeNumTypePath, PreludeTypePath};
use husky_eth_signature::{
    helpers::trai_for_ty::is_ty_term_always_copyable, signature::HasEthTemplate,
};
use husky_eth_term::term::{
    application::{EthApplication, TermFunctionReduced},
    symbolic_variable::EthTermVariableIndexImpl,
    EthTerm,
};
use husky_fly_term::{
    data::sol::SolTermData, quary::FlyQuary, FlyTerm, FlyTermBase, FlyTerms, SolTerm, SolTerms,
};
use husky_term_prelude::ItemPathTerm;
use path::major_item::form::MajorFormPath;

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
    TypeVar(MajorFormPath),
    Quaried,
}

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypeTypeAssocType {}

#[salsa::interned(jar = HirTypeJar)]
pub struct HirTypeTraitAssocType {}

impl HirType {
    pub fn from_eth(term: EthTerm, db: &::salsa::Db) -> Option<Self> {
        let always_copyable = is_ty_term_always_copyable(term, db).unwrap()?;
        match term {
            EthTerm::SymbolicVariable(variable) => {
                HirTypeTemplateVariable::from_eth(variable, db).map(Into::into)
            }
            EthTerm::ItemPath(path) => match path {
                ItemPathTerm::MajorForm(path) => match path.kind(db) {
                    MajorFormKind::Ritchie(_) => todo!(),
                    MajorFormKind::TypeAlias => todo!(),
                    MajorFormKind::TypeVar => Some(HirType::TypeVar(path)),
                    MajorFormKind::Val => todo!(),
                    MajorFormKind::StaticMut => todo!(),
                    MajorFormKind::StaticVar => todo!(),
                    MajorFormKind::Compterm => todo!(),
                    MajorFormKind::Conceptual => todo!(),
                },
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

    /// this will ignore the quary
    pub fn from_fly_base(term: FlyTerm, db: &::salsa::Db, terms: &FlyTerms) -> Option<Self> {
        match term.base_resolved_inner(terms) {
            FlyTermBase::Eth(term) => HirType::from_eth(term, db),
            FlyTermBase::Sol(term) => HirType::from_sol(term, db, terms),
            FlyTermBase::Hol(_) => unreachable!("expected all fly terms to be resolved"),
            FlyTermBase::Place => todo!(),
        }
    }

    pub fn from_sol(term: SolTerm, db: &::salsa::Db, terms: &FlyTerms) -> Option<Self> {
        let sol_terms = terms.sol_terms();
        let always_copyable = <_ as Into<FlyTerm>>::into(term)
            .always_copyable(db, terms)
            .unwrap()?;
        match *term.data2(sol_terms) {
            SolTermData::TypeOntology {
                path,
                ref arguments,
                ..
            } => {
                let template_arguments = arguments
                    .iter()
                    .map(|&arg| match Self::from_fly(arg, db, terms) {
                        Some(ty_arg) => ty_arg.into(),
                        None => todo!(),
                    })
                    .collect();
                Some(HirTypePathLeading::new(db, path, template_arguments, always_copyable).into())
            }
            SolTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => todo!(),
            SolTermData::Ritchie {
                ritchie_kind,
                ref parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }

    pub fn from_fly(term: FlyTerm, db: &::salsa::Db, terms: &FlyTerms) -> Option<Self> {
        let base_ty = Self::from_fly_base(term, db, terms);
        if let Some(quary) = term.quary() {
            match quary {
                FlyQuary::Compterm => todo!(),
                FlyQuary::StackPure { place } => todo!(),
                FlyQuary::ImmutableOnStack { place } => todo!(),
                FlyQuary::MutableOnStack { place } => Some(HirType::Quaried),
                FlyQuary::Transient => base_ty,
                FlyQuary::Ref { guard } => todo!(),
                FlyQuary::RefMut { place, lifetime } => todo!(),
                FlyQuary::Leashed { place } => todo!(),
                FlyQuary::Todo => todo!(),
                FlyQuary::EtherealSymbol(_) => todo!(),
            }
        } else {
            base_ty
        }
    }

    pub fn prelude_ty_path(self, db: &::salsa::Db) -> Option<PreludeTypePath> {
        match self {
            HirType::PathLeading(hir_ty) => hir_ty.ty_path(db).prelude(db),
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
            HirType::TypeVar(_) => false,
            HirType::Quaried => todo!(), // ad hoc: todo check traits
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
                match param.variable().index(db).inner() {
                    EthTermVariableIndexImpl::ExplicitLifetime { attrs, .. }
                    | EthTermVariableIndexImpl::ExplicitPlace { attrs, .. }
                    | EthTermVariableIndexImpl::Type { attrs, .. }
                    | EthTermVariableIndexImpl::ConstOther { attrs, .. }
                    | EthTermVariableIndexImpl::ConstPathLeading { attrs, .. } => !attrs.phantom(),
                    EthTermVariableIndexImpl::Prop { .. } => false,
                    EthTermVariableIndexImpl::EphemPathLeading { .. }
                    | EthTermVariableIndexImpl::EphemOther { .. }
                    | EthTermVariableIndexImpl::SelfType
                    | EthTermVariableIndexImpl::SelfValue
                    | EthTermVariableIndexImpl::SelfLifetime
                    | EthTermVariableIndexImpl::SelfPlace => unreachable!(),
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
        TermFunctionReduced::TypeVar(_) => todo!(),
        TermFunctionReduced::Trait(_) => todo!(),
        TermFunctionReduced::Other(_) => todo!(),
    }
}
