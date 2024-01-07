use crate::{path_leading::HirTypePathLeading, ritchie::HirRitchieType, *};
use husky_ethereal_signature::helpers::trai_for_ty::is_ty_term_always_copyable;
use husky_ethereal_term::{EtherealTerm};
use husky_fluffy_term::{FluffyTerm, FluffyTermBase, FluffyTerms};
use husky_term_prelude::TermEntityPath;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateArgument {
    /// `Vacant` is used to repr abstract types
    ///
    /// say a list of any element type
    ///
    /// It doesn't mean two elements in the list can be of different type
    ///
    /// It just means that the type is capable of representing any list,
    /// saving the need to recompile.
    ///
    /// It should be noted that phantom template parameter should only accept vacant parameter.
    Vacant,
    Type(HirType),
    Constant(HirConstant),
    Lifetime(HirLifetimeSymbol),
    Place(HirPlaceSymbol),
}

impl From<HirTemplateSymbol> for HirTemplateArgument {
    fn from(symbol: HirTemplateSymbol) -> Self {
        match symbol {
            HirTemplateSymbol::Type(symbol) => HirTemplateArgument::Type(symbol.into()),
            HirTemplateSymbol::Const(symbol) => HirTemplateArgument::Constant(symbol.into()),
            HirTemplateSymbol::Lifetime(symbol) => HirTemplateArgument::Lifetime(symbol),
            HirTemplateSymbol::Place(symbol) => HirTemplateArgument::Place(symbol),
        }
    }
}

pub type HirTemplateArguments = smallvec::SmallVec<[HirTemplateArgument; 2]>;

impl HirTemplateArgument {
    pub(crate) fn from_ethereal(argument: EtherealTerm, db: &::salsa::Db) -> Option<Self> {
        Some(match argument {
            EtherealTerm::Literal(lit) => HirConstant::from_term(lit, db).into(),
            EtherealTerm::Symbol(symbol) => HirTemplateSymbol::from_ethereal(symbol, db)?.into(),
            EtherealTerm::Rune(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_path) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(ty_path) => {
                    let always_copyable = is_ty_term_always_copyable(argument, db).unwrap()?;
                    HirTemplateArgument::Type(
                        HirTypePathLeading::new(db, ty_path, Default::default(), always_copyable)
                            .into(),
                    )
                }
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(path) => HirTemplateArgument::Constant(path.into()),
            },
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(term) => {
                HirType::Ritchie(HirRitchieType::from_ethereal(term, db)).into()
            }
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(application) => {
                hir_ty_from_ethereal_term_application(db, application).into()
            }
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        })
    }

    pub(crate) fn from_fluffy(
        term: FluffyTerm,
        db: &::salsa::Db,
        terms: &FluffyTerms,
    ) -> Option<Self> {
        match term.base_resolved_inner(terms) {
            FluffyTermBase::Ethereal(ethereal_term) => Self::from_ethereal(ethereal_term, db),
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(t) => {
                use husky_print_utils::p;
                p!(t, term.show(db, terms));
                todo!()
            }
            FluffyTermBase::Place => todo!(),
        }
    }
}
