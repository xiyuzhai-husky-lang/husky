use super::*;
use husky_print_utils::p;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub(crate) enum LocalTermExpectation {
    None,
    Type,
    Condition,
    Return { ty: Option<ReducedTerm> },
    ImplicitlyConvertibleTo { term: LocalTerm },
}

impl LocalTermExpectation {
    pub(crate) fn term(self) -> Option<ReducedTerm> {
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
    expectee: LocalTerm,
    variant: ExpectationRuleVariant,
    resolve_progress: LocalTermResolveProgress,
}

impl LocalTermExpectationRule {
    pub(crate) fn new(
        db: &dyn ExprTypeDb,
        reduced_term_menu: ReducedTermMenu,
        target: LocalTerm,
        variant: ExpectationRuleVariant,
    ) -> Self {
        match target {
            LocalTerm::Resolved(target) => {
                Self::new_resolved(db, reduced_term_menu, target, variant)
            }
            LocalTerm::Unresolved(target) => Self::new_unresolved(target, variant),
        }
    }

    fn new_resolved(
        db: &dyn ExprTypeDb,
        reduced_term_menu: ReducedTermMenu,
        resolved_term: ReducedTerm,
        variant: ExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        let resolve_progress = match variant {
            ExpectationRuleVariant::AsBool => match resolved_term {
                term if term == reduced_term_menu.bool() => LocalTermResolveProgress::Resolved {
                    implicit_conversion: LocalTermImplicitConversion::None,
                    term,
                },
                // MOM
                term if term == reduced_term_menu.i32() => todo!(),
                term if term == reduced_term_menu.r32() => todo!(),
                term => todo!(),
            },
            ExpectationRuleVariant::ImplicitlyConvertibleTo { dst: term } => match term {
                LocalTerm::Resolved(_) => todo!(),
                LocalTerm::Unresolved(_) => LocalTermResolveProgress::Unresolved,
            },
            ExpectationRuleVariant::Type => match db.term_ty(resolved_term.term()) {
                Ok(term_ty) => match term_ty.term() {
                    Term::Category(cat) => match cat.universe().raw() {
                        0 => todo!(),
                        _ => LocalTermResolveProgress::Resolved {
                            implicit_conversion: LocalTermImplicitConversion::None,
                            term: resolved_term,
                        },
                    },
                    _ => todo!(),
                },
                Err(_) => todo!(),
            },
        };
        Self {
            expectee: resolved_term.into(),
            variant,
            resolve_progress,
        }
    }
    fn new_unresolved(
        target: UnresolvedTermIdx,
        variant: ExpectationRuleVariant,
    ) -> LocalTermExpectationRule {
        Self {
            expectee: target.into(),
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

    pub(crate) fn target(&self) -> LocalTerm {
        self.expectee
    }

    pub(super) fn action(
        &self,
        table: &UnresolvedTermTable,
        level: LocalTermResolveLevel,
    ) -> Option<TermResolveAction> {
        match self.variant {
            ExpectationRuleVariant::AsBool => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => todo!(),
            },
            ExpectationRuleVariant::ImplicitlyConvertibleTo { dst } => match self.expectee {
                LocalTerm::Resolved(_) => match dst {
                    LocalTerm::Resolved(_) => todo!(),
                    LocalTerm::Unresolved(dst) => match table[dst].pattern() {
                        UnresolvedTermPattern::ImplicitSymbol => match level {
                            LocalTermResolveLevel::Weak => todo!(),
                            LocalTermResolveLevel::Strong => todo!(),
                        },
                        UnresolvedTermPattern::Injection {
                            function,
                            arguments,
                        } => todo!(),
                    },
                },
                LocalTerm::Unresolved(expectee) => match table[expectee].pattern() {
                    UnresolvedTermPattern::ImplicitSymbol => match level {
                        LocalTermResolveLevel::Weak => None,
                        LocalTermResolveLevel::Strong => todo!(),
                    },
                    UnresolvedTermPattern::Injection {
                        function,
                        arguments,
                    } => todo!(),
                },
            },
            ExpectationRuleVariant::Type => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum ExpectationRuleVariant {
    AsBool,
    ImplicitlyConvertibleTo { dst: LocalTerm },
    Type,
}
