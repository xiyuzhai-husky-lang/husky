use crate::{path_leading::HirTypePathLeading, quary::HirQuary, ritchie::HirRitchieType, *};
use husky_eth_signature::helpers::trai_for_ty::is_ty_term_always_copyable;
use husky_eth_term::term::EthTerm;
use husky_fly_term::{FlyTerm, FlyTermBase, FlyTerms};
use husky_term_prelude::ItemPathTerm;

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
    Lifetime(HirLifetimeSvar),
    Quary(HirQuary),
}

impl From<HirTemplateSvar> for HirTemplateArgument {
    fn from(symbol: HirTemplateSvar) -> Self {
        match symbol {
            HirTemplateSvar::Type(symbol) => HirTemplateArgument::Type(symbol.into()),
            HirTemplateSvar::Const(symbol) => HirTemplateArgument::Constant(symbol.into()),
            HirTemplateSvar::Lifetime(symbol) => HirTemplateArgument::Lifetime(symbol.into()),
            HirTemplateSvar::Quary(symbol) => HirTemplateArgument::Quary(symbol.into()),
        }
    }
}

pub type HirTemplateArguments = smallvec::SmallVec<[HirTemplateArgument; 2]>;

impl HirTemplateArgument {
    pub(crate) fn from_eth(argument: EthTerm, db: &::salsa::Db) -> Option<Self> {
        Some(match argument {
            EthTerm::Literal(lit) => HirConstant::from_term(lit, db).into(),
            EthTerm::Symbol(symbol) => HirTemplateSvar::from_eth(symbol, db)?.into(),
            EthTerm::Hvar(_) => todo!(),
            EthTerm::EntityPath(path) => match path {
                ItemPathTerm::Fugitive(_path) => todo!(),
                ItemPathTerm::Trait(_) => todo!(),
                ItemPathTerm::TypeOntology(ty_path) => {
                    let always_copyable = is_ty_term_always_copyable(argument, db).unwrap()?;
                    HirTemplateArgument::Type(
                        HirTypePathLeading::new(db, ty_path, Default::default(), always_copyable)
                            .into(),
                    )
                }
                ItemPathTerm::TypeInstance(_) => todo!(),
                ItemPathTerm::TypeVariant(path) => HirTemplateArgument::Constant(path.into()),
            },
            EthTerm::Category(_) => todo!(),
            EthTerm::Universe(_) => todo!(),
            EthTerm::Curry(_) => todo!(),
            EthTerm::Ritchie(term) => HirType::Ritchie(HirRitchieType::from_eth(term, db)).into(),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(application) => {
                hir_ty_from_eth_term_application(db, application).into()
            }
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        })
    }

    pub(crate) fn from_fly(term: FlyTerm, db: &::salsa::Db, terms: &FlyTerms) -> Option<Self> {
        match term.base_resolved_inner(terms) {
            FlyTermBase::Eth(ethereal_term) => Self::from_eth(ethereal_term, db),
            FlyTermBase::Sol(_) => todo!(),
            FlyTermBase::Hol(t) => {
                use husky_print_utils::p;
                p!(t, term.show(db, terms));
                todo!()
            }
            FlyTermBase::Place => todo!(),
        }
    }
}
