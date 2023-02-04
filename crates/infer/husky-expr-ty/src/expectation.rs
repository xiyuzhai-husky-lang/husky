use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Expectation {
    None,
    Type,
    UnitOrNever,
    Condition,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExpectationRule {
    variant: ExpectationRuleVariant,
    resolve_progress: ExpectationRuleResolveProgress,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExpectationRuleVariant {}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpectationRuleResolveProgress {
    Unresolved,
    Ok,
    Err(OriginalTypeError),
}
