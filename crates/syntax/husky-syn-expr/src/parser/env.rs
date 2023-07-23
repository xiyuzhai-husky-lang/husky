use super::*;
use husky_opr::Bracket;
use husky_print_utils::p;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprEnvironment {
    TypeBeforeEq,
    WithinBracketedParameterList(Bracket),
    Condition(TokenIdxRangeEnd),
}

pub struct ExprEnvironmentStack(smallvec::SmallVec<[ExprEnvironment; 2]>);

impl ExprEnvironmentStack {
    pub(super) fn new(env: Option<ExprEnvironment>) -> Self {
        ExprEnvironmentStack(env.into_iter().collect())
    }

    pub(super) fn set(&mut self, env: ExprEnvironment) {
        self.0.push(env)
    }

    pub(super) fn unset(&mut self) {
        self.0.pop().expect("len() > 0");
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn env(&self) -> Option<ExprEnvironment> {
        self.env_stack.0.last().copied()
    }
    pub(super) fn env_bra(&self) -> Option<Bracket> {
        match self.env()? {
            ExprEnvironment::WithinBracketedParameterList(bra) => Some(bra),
            ExprEnvironment::TypeBeforeEq => None,
            ExprEnvironment::Condition(_) => None,
        }
    }
}
