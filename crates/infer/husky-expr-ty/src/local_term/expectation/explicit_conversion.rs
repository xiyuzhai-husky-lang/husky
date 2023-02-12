use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone)]
pub(crate) struct ExpectExplicitConversion {
    destination: LocalTerm,
}

pub(crate) struct ExpectExplicitConversionResult {}

impl From<ExpectExplicitConversion> for LocalTermExpectationRuleVariant {
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
