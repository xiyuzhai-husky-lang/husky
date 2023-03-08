use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

impl ExpectExplicitlyConvertible {
    pub(in super::super) fn try_substitute_unresolved_local_term<'a>(
        &self,
        unresolved_terms: &'a UnresolvedTerms,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match unresolved_terms.try_reduce_local_term(self.destination)? {
            Some(destination) => Ok(Some(ExpectExplicitlyConvertible { destination }.into())),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitlyConvertibleOutcome {
    destination: LocalTerm,
}

impl ExpectLocalTermOutcome for ExpectExplicitlyConvertibleOutcome {
    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::ExplicitlyConvertible(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl ExpectLocalTerm for ExpectExplicitlyConvertible {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        todo!()
    }
}

// LocalTermExpectationRuleVariant::AsBool => {
//     match resolved_term {
//         term if term == reduced_term_menu.bool() => {
//             LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationOutcome::OkExplicitConversion {
//                     local_term: term.into(),
//                     implicit_conversion: LocalTermImplicitConversion::None,
//                 },
//             )
//         }
//         // MOM
//         term if term == reduced_term_menu.i32() => todo!(),
//         term if term == reduced_term_menu.r32() => todo!(),
//         term => todo!(),
//     }
// }

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_explicitly_convertible(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectExplicitlyConvertible,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        // todo
        None
    }
}
