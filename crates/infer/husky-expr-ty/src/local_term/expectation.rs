use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectation {
    None,
    Type,
    Condition,
    Return { ty: Option<Term> },
    ImplicitlyConvertibleTo { term: LocalTerm },
}

impl LocalTermExpectation {
    pub(crate) fn term(self) -> Option<Term> {
        match self {
            LocalTermExpectation::None => None,
            LocalTermExpectation::Type => None,
            LocalTermExpectation::Condition => None,
            LocalTermExpectation::Return { ty } => ty,
            LocalTermExpectation::ImplicitlyConvertibleTo { term } => term.resolved(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LocalTermExpectationRule {
    target: LocalTerm,
    variant: ExpectationRuleVariant,
    resolve_progress: LocalTermResolveProgress,
}

impl LocalTermExpectationRule {
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
        term: Term,
        variant: ExpectationRuleVariant,
        term_menu: &TermMenu,
    ) -> LocalTermExpectationRule {
        let resolve_progress = match variant {
            ExpectationRuleVariant::Condition => match term {
                term if term == term_menu.bool() => LocalTermResolveProgress::Resolved {
                    implicit_conversion: LocalTermImplicitConversion::None,
                    term,
                },
                // MOM
                term if term == term_menu.i32() => todo!(),
                term if term == term_menu.r32() => todo!(),
                term => todo!(),
            },
            ExpectationRuleVariant::ImplicitlyConvertibleTo { term } => todo!(),
        };
        Self {
            target: term.into(),
            variant,
            resolve_progress,
        }
    }
    fn new_unresolved(
        target: UnresolvedTermIdx,
        variant: ExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        Self {
            target: target.into(),
            variant,
            resolve_progress: LocalTermResolveProgress::Unresolved,
        }
    }

    pub(crate) fn variant(&self) -> &ExpectationRuleVariant {
        &self.variant
    }

    pub(crate) fn resolve_progress(&self) -> &LocalTermResolveProgress {
        &self.resolve_progress
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum ExpectationRuleVariant {
    Condition,
    ImplicitlyConvertibleTo { term: LocalTerm },
}
