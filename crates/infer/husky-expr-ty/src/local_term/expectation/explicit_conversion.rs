use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone)]
pub(crate) struct ExpectExplicitConversion {
    destination: LocalTerm,
}

pub(crate) struct ExpectExplicitConversionResult {}

impl From<ExpectExplicitConversion> for LocalTermExpectation {
    fn from(value: ExpectExplicitConversion) -> Self {
        todo!()
    }
}

impl From<ExpectExplicitConversionResult> for LocalTermExpectationResult {
    fn from(value: ExpectExplicitConversionResult) -> Self {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectExplicitConversion {
    type Result = ExpectExplicitConversionResult;

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

// LocalTermExpectationRuleVariant::AsBool => {
//     match resolved_term {
//         term if term == reduced_term_menu.bool() => {
//             LocalTermExpectationResolveProgress::Resolved(
//                 LocalTermExpectationResult::OkExplicitConversion {
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
