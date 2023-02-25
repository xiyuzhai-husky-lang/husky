use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

impl const ProvideTypeContext for ExpectExplicitlyConvertible {
    fn ty_context(&self) -> TypeContext {
        Default::default()
    }
}

impl ExpectExplicitlyConvertible {
    pub(in super::super) fn try_substitute_unresolved_local_term<'a>(
        &self,
        unresolved_terms: &'a UnresolvedTerms,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match unresolved_terms.try_substitute_local_term(self.destination)? {
            Some(destination) => Ok(Some(ExpectExplicitlyConvertible { destination }.into())),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitlyConvertibleResolvedOk {
    destination: LocalTerm,
}

impl ExpectLocalTermResolvedOk for ExpectExplicitlyConvertibleResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::ExplicitlyConvertible(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl From<ExpectExplicitlyConvertible> for LocalTermExpectation {
    fn from(value: ExpectExplicitlyConvertible) -> Self {
        LocalTermExpectation::ExplicitlyConvertible(value)
    }
}

impl From<ExpectExplicitlyConvertibleResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectExplicitlyConvertibleResolvedOk) -> Self {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectExplicitlyConvertible {
    type ResolvedOk = ExpectExplicitlyConvertibleResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

// LocalTermExpectationRuleVariant::AsBool => {
//     match resolved_term {
//         term if term == reduced_term_menu.bool() => {
//             LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationResolvedOk::OkExplicitConversion {
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
        destination: LocalTerm,
        level: LocalTermResolveLevel,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        // todo
        None
    }
}
