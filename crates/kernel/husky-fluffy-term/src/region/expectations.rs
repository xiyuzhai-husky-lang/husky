use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSource {
    ExpectationResolve { parent: FluffyTermExpectationIdx },
}
