mod hollow_terms;
mod solid_terms;

pub use self::hollow_terms::*;
pub use self::solid_terms::*;

use super::*;

// `Default` is not implemented because we might need to initialize `solid_terms` from the parent
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct FluffyTerms {
    solid_terms: SolidTerms,
    hollow_terms: HollowTerms,
}

impl std::borrow::Borrow<HollowTerms> for FluffyTerms {
    fn borrow(&self) -> &HollowTerms {
        &self.hollow_terms
    }
}

impl FluffyTerms {
    pub(crate) fn new(terms: Option<&Self>) -> Self {
        Self {
            solid_terms: SolidTerms::new(terms.map(|terms| &terms.solid_terms)),
            // `Default` is derived for `hollow_terms` because we never inherited hollow terms
            hollow_terms: Default::default(),
        }
    }

    pub(crate) fn new_hole_from_parameter_rune(
        &mut self,
        db: &::salsa::Db,
        hole_source: HoleSource,
        parameter_rune: FluffyTermRune,
    ) -> HollowTerm {
        let hole_kind = match parameter_rune.data_inner(db, self) {
            FluffyTermData::Rune { ty } => match ty.data_inner(db, self) {
                FluffyTermData::TypeOntology {
                    ty_path: path,
                    refined_ty_path: refined_path,
                    ty_arguments: arguments,
                    ty_ethereal_term,
                } => todo!(),
                FluffyTermData::Curry {
                    curry_kind,
                    variance,
                    parameter_rune: parameter_rune,
                    parameter_ty,
                    return_ty,
                    ty_ethereal_term,
                } => todo!(),
                FluffyTermData::Hole(_, _) => todo!(),
                FluffyTermData::Category(cat) => {
                    if cat.universe().raw() != 1 {
                        // maybe we need to consider universe
                        todo!()
                    }
                    HoleKind::ImplicitType
                }
                FluffyTermData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                } => todo!(),
                FluffyTermData::Symbol { .. } => HoleKind::Any,
                FluffyTermData::Rune { ty } => todo!(),
                _ => unreachable!(),
            },
            FluffyTermData::Hole(_, _) => todo!(),
            // match parameter_symbol.base() {
            //     FluffyTermBase::Ethereal(_) => todo!(),
            //     FluffyTermBase::Solid(_) => todo!(),
            //     FluffyTermBase::Hollow(hole_term) => return hole_term,
            //     FluffyTermBase::Place => todo!(),
            // },
            _ => {
                p!(parameter_rune.data_inner(db, self).debug(db));
                unreachable!()
            }
        };
        self.hollow_terms.alloc_new(HollowTermData::Hole {
            hole_source,
            hole_kind,
            fill: None,
            constraints: smallvec![],
        })
    }

    pub fn hollow_terms(&self) -> &HollowTerms {
        &self.hollow_terms
    }

    pub fn solid_terms(&self) -> &SolidTerms {
        &self.solid_terms
    }

    pub fn hollow_terms_mut(&mut self) -> &mut HollowTerms {
        &mut self.hollow_terms
    }

    pub fn solid_terms_mut(&mut self) -> &mut SolidTerms {
        &mut self.solid_terms
    }
}
