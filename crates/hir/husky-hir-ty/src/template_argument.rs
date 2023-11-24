use crate::{ritchie::HirRitchieType, *};
use husky_ethereal_term::{EtherealTerm, EtherealTermSymbolIndexInner};
use husky_fluffy_term::{FluffyTerm, FluffyTermBase, FluffyTerms};
use husky_term_prelude::TermEntityPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = HirTypeDb)]
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

impl From<HirComptimeSymbol> for HirTemplateArgument {
    fn from(symbol: HirComptimeSymbol) -> Self {
        match symbol {
            HirComptimeSymbol::Type(symbol) => HirTemplateArgument::Type(symbol.into()),
            HirComptimeSymbol::Const(symbol) => HirTemplateArgument::Constant(symbol.into()),
            HirComptimeSymbol::Lifetime(symbol) => HirTemplateArgument::Lifetime(symbol),
            HirComptimeSymbol::Place(symbol) => HirTemplateArgument::Place(symbol),
        }
    }
}

pub type HirTemplateArguments = smallvec::SmallVec<[HirTemplateArgument; 2]>;

// .then(|| HirTemplateArgument::Type(HirType::from_ethereal(arg, db))),
impl HirTemplateArgument {
    pub(crate) fn from_ethereal(argument: EtherealTerm, db: &dyn HirTypeDb) -> Option<Self> {
        Some(match argument {
            EtherealTerm::Literal(lit) => HirConstant::from_term(lit, db).into(),
            EtherealTerm::Symbol(symbol) => HirComptimeSymbol::from_ethereal(symbol, db)?.into(),
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(path) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(ty_path) => HirTemplateArgument::Type(
                    HirTypePathLeading::new(db, ty_path, Default::default()).into(),
                ),
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
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
        fluffy_term: FluffyTerm,
        db: &dyn HirTypeDb,
        fluffy_terms: &FluffyTerms,
    ) -> Option<Self> {
        match fluffy_term.base_resolved_inner(fluffy_terms) {
            FluffyTermBase::Ethereal(ethereal_term) => Self::from_ethereal(ethereal_term, db),
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(_) => todo!(),
            FluffyTermBase::Place => todo!(),
        }
    }
}
