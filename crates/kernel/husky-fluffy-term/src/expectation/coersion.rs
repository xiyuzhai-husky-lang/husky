//! coersion rules are
//!
mod deref;
mod never;
mod place_to_prelude_indirection;
mod trival;
mod wrap_in_some;

use self::trival::PlaceCoersion;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Coersion {
    Place(PlaceCoersion),
    Never,
    Other,
    WrapInSome,
}

/// expect a type that is implicitly convertible to type under contract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct ExpectCoersion {
    contract: Contract,
    ty_expected: FluffyTerm,
}

impl ExpectCoersion {
    #[inline(always)]
    pub fn new(contract: Contract, ty_expected: FluffyTerm) -> Self {
        Self {
            contract,
            ty_expected,
        }
    }

    #[inline(always)]
    pub fn new_const(ty: FluffyTerm) -> Self {
        Self {
            contract: Contract::Const,
            ty_expected: ty,
        }
    }

    /// this will reduce place type to type
    #[inline(always)]
    pub fn new_pure(engine: &impl FluffyTermEngine, ty: FluffyTerm) -> Self {
        let ty = match ty.data(engine) {
            FluffyTermData::TypeOntologyAtPlace {
                ty_path: path,
                ty_arguments: arguments,
                ..
            } => match arguments.len() {
                0 => TermEntityPath::TypeOntology(path).into(),
                _ => todo!(),
            },
            _ => ty,
        };
        Self {
            contract: Contract::None,
            ty_expected: ty,
        }
    }

    #[inline(always)]
    pub fn new_pure_unit(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: Contract::None,
            ty_expected: engine.term_menu().unit_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_pure_bool(engine: &impl FluffyTermEngine) -> Self {
        Self {
            contract: Contract::None,
            ty_expected: engine.term_menu().bool_ty_ontology().into(),
        }
    }

    #[inline(always)]
    pub fn new_move(ty: FluffyTerm) -> Self {
        Self {
            contract: Contract::Move,
            ty_expected: ty,
        }
    }

    pub(crate) fn try_substitute_unresolved_fluffy_term<'a>(
        &self,
        terms: &'a FluffyTerms,
    ) -> Result<Option<Expectation>, &'a HollowTermResolveError> {
        todo!()
        // match terms.try_reduce_fluffy_term(self.expected)? {
        //     Some(destination) => Ok(Some(
        //         ExpectImplicitlyConvertible {
        //             expected: destination,
        //         }
        //         .into(),
        //     )),
        //     None => Ok(None),
        // }
    }

    fn contract(self) -> Contract {
        self.contract
    }

    fn ty(self) -> FluffyTerm {
        self.ty_expected
    }
}

impl ExpectFluffyTerm for ExpectCoersion {
    type Outcome = Coersion;

    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::ImplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        self.ty().final_destination_inner(db, terms)
    }

    fn destination(&self) -> Option<FluffyTerm> {
        Some(self.ty())
    }

    fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        self.resolve_trivial(db, terms, state)?;
        self.resolve_never(db, terms, state)?;
        self.resolve_wrap_in_some(db, terms, state)?;
        self.resolve_deref(db, terms, state)?;
        self.resolve_place_to_prelude_indirection(db, terms, state)?;
        state.set_err(
            OriginalFluffyTermExpectationError::ExpectedCoersion {
                expectee: state.expectee(),
                contract: self.contract,
                expected: self.ty_expected,
            },
            smallvec![],
        )
    }
}

// common auxiliary
fn resolve_aux(
    src: FluffyBaseTypeData,
    dst: FluffyBaseTypeData,
    coersion: Coersion,
    db: &dyn FluffyTermDb,
    terms: &mut FluffyTerms,
    state: &mut ExpectationState,
) -> AltOption<ExpectationEffect> {
    if src == dst {
        return state.set_ok(coersion, smallvec![]);
    }
    match src {
        FluffyBaseTypeData::TypeOntology {
            ty_path: src_ty_path,
            ty_arguments: src_ty_arguments,
            ..
        } => todo!(),
        FluffyBaseTypeData::Curry {
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
            ty_ethereal_term,
        } => todo!(),
        FluffyBaseTypeData::Hole(_, _) => todo!(),
        FluffyBaseTypeData::Category(_) => todo!(),
        FluffyBaseTypeData::Ritchie {
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        } => todo!(),
        FluffyBaseTypeData::Symbol { term } => todo!(),
        FluffyBaseTypeData::Variable { ty } => todo!(),
        FluffyBaseTypeData::TypeVariant { path } => todo!(),
    }
}
