use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub(crate) enum Expectation {
    None,
    Type,
    UnitOrNever,
    Condition,
    Return { ty: Option<Term> },
    MoveAs { ty: LocalTerm },
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExpectationRule {
    target: LocalTerm,
    variant: ExpectationRuleVariant,
    resolve_progress: ExpectationRuleResolveProgress,
}

impl ExpectationRule {
    pub(crate) fn new(
        target: LocalTerm,
        variant: ExpectationRuleVariant,
        term_menu: &TermMenu,
    ) -> Self {
        match target {
            LocalTerm::Resolved(target) => Self::new_resolved(target, variant, term_menu),
            LocalTerm::Unresolved(target) => Self::new_unresolved(target, variant),
        }
    }

    fn new_resolved(
        target: Term,
        variant: ExpectationRuleVariant,
        term_menu: &TermMenu,
    ) -> ExpectationRule {
        let resolve_progress = match variant {
            ExpectationRuleVariant::Condition => match target {
                target if target == term_menu.bool() => ExpectationRuleResolveProgress::Ok,
                target if target == term_menu.i32() => todo!(),
                target if target == term_menu.r32() => todo!(),
                target => todo!(),
            },
        };
        Self {
            target: target.into(),
            variant,
            resolve_progress,
        }
    }
    fn new_unresolved(
        target: UnresolvedTermIdx,
        variant: ExpectationRuleVariant,
    ) -> ExpectationRule {
        Self {
            target: target.into(),
            variant,
            resolve_progress: ExpectationRuleResolveProgress::Unresolved,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpectationRuleVariant {
    Condition,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpectationRuleResolveProgress {
    Unresolved,
    Ok,
    Err(OriginalTypeError),
}
