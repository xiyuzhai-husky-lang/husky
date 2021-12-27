use std::sync::Arc;

use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprError {
    range: TextRange,
    kind: ExprKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprKind {
    BreakRule(ExprRule),
    AtomError(atom::AtomErrorKind),
}

impl ExprError {
    pub fn new(range: TextRange, rule_broken: ExprRule) -> ExprError {
        Self {
            range,
            kind: ExprKind::BreakRule(rule_broken),
        }
    }
}

pub type ExprResult<T> = Result<T, ExprError>;
pub type ExprResultArc<T> = Result<Arc<T>, ExprError>;

impl From<&atom::AtomError> for ExprError {
    fn from(error: &atom::AtomError) -> Self {
        Self {
            range: error.range.clone(),
            kind: ExprKind::AtomError(error.kind.clone()),
        }
    }
}

impl From<atom::AtomError> for ExprError {
    fn from(error: atom::AtomError) -> Self {
        Self {
            range: error.range,
            kind: ExprKind::AtomError(error.kind),
        }
    }
}

impl From<scope::ModuleFromFileError> for ExprError {
    fn from(_: scope::ModuleFromFileError) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExprRule {
    ScopeShouldBeCalled,
    BracketsShouldMatch,
    BracketedExprCommaListShouldBeEitherCalledOrIndexed,
}
