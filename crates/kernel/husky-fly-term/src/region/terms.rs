mod hol_terms;
mod sol_terms;

pub use self::hol_terms::*;
pub use self::sol_terms::*;

use super::*;

// `Default` is not implemented because we might need to initialize `solid_terms` from the parent
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct FlyTerms {
    solid_terms: SolTerms,
    hollow_terms: HolTerms,
}

impl std::borrow::Borrow<HolTerms> for FlyTerms {
    fn borrow(&self) -> &HolTerms {
        &self.hollow_terms
    }
}

impl FlyTerms {
    pub(crate) fn new(terms: Option<&Self>) -> Self {
        Self {
            solid_terms: SolTerms::new(terms.map(|terms| &terms.solid_terms)),
            // `Default` is derived for `hollow_terms` because we never inherited hollow terms
            hollow_terms: Default::default(),
        }
    }

    pub(crate) fn new_hole_from_template_parameter_symbol(
        &mut self,
        hole_source: HoleSource,
        template_parameter_symbol: SymbolEthTerm,
        db: &::salsa::Db,
    ) -> HolTerm {
        let hole_kind = match template_parameter_symbol.ty(db) {
            EthTerm::Literal(_) => todo!(),
            EthTerm::Symbol(_) => HoleKind::Any,
            EthTerm::Rune(_) => todo!(),
            EthTerm::EntityPath(_) => todo!(),
            EthTerm::Category(cat) => {
                if cat.universe().raw() != 1 {
                    // maybe we need to consider universe
                    todo!()
                }
                HoleKind::ImplicitType
            }
            EthTerm::Universe(_) => todo!(),
            EthTerm::Curry(_) => todo!(),
            EthTerm::Ritchie(_) => todo!(),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::Application(_) => todo!(),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        };
        self.hollow_terms.alloc_new(HolTermData::Hole {
            hole_source,
            hole_kind,
            fill: None,
            constraints: smallvec![],
        })
    }

    #[deprecated(note = "create holes from template parameters directly instead")]
    pub(crate) fn new_hole_from_parameter_rune(
        &mut self,
        db: &::salsa::Db,
        hole_source: HoleSource,
        parameter_rune: RuneFlyTerm,
    ) -> HolTerm {
        let hole_kind = match parameter_rune.data_inner(db, self) {
            FlyTermData::Rune { ty, .. } => match ty.data_inner(db, self) {
                FlyTermData::TypeOntology {
                    ty_path: path,
                    refined_ty_path: refined_path,
                    ty_arguments: arguments,
                    ty_ethereal_term,
                } => todo!(),
                FlyTermData::Curry {
                    toolchain,
                    curry_kind,
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                    ty_ethereal_term,
                } => todo!(),
                FlyTermData::Hole(_, _) => todo!(),
                FlyTermData::Category(cat) => {
                    if cat.universe().raw() != 1 {
                        // maybe we need to consider universe
                        todo!()
                    }
                    HoleKind::ImplicitType
                }
                FlyTermData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                } => todo!(),
                FlyTermData::Symbol { .. } => HoleKind::Any,
                FlyTermData::Rune { .. } => todo!(),
                _ => unreachable!(),
            },
            FlyTermData::Hole(_, _) => todo!(),
            // match parameter_symbol.base() {
            //     FlyTermBase::Ethereal(_) => todo!(),
            //     FlyTermBase::Solid(_) => todo!(),
            //     FlyTermBase::Hollow(hole_term) => return hole_term,
            //     FlyTermBase::Place => todo!(),
            // },
            _ => {
                p!(parameter_rune.data_inner(db, self).debug(db));
                unreachable!()
            }
        };
        self.hollow_terms.alloc_new(HolTermData::Hole {
            hole_source,
            hole_kind,
            fill: None,
            constraints: smallvec![],
        })
    }

    pub fn hollow_terms(&self) -> &HolTerms {
        &self.hollow_terms
    }

    pub fn solid_terms(&self) -> &SolTerms {
        &self.solid_terms
    }

    pub fn hollow_terms_mut(&mut self) -> &mut HolTerms {
        &mut self.hollow_terms
    }

    pub fn solid_terms_mut(&mut self) -> &mut SolTerms {
        &mut self.solid_terms
    }
}
