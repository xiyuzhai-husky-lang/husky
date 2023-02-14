use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectExplicitConvertible {
    destination: LocalTerm,
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

impl From<ExpectExplicitConvertible> for LocalTermExpectation {
    fn from(value: ExpectExplicitConvertible) -> Self {
        todo!()
    }
}

impl From<ExpectExplicitlyConvertibleResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectExplicitlyConvertibleResolvedOk) -> Self {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectExplicitConvertible {
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
