use super::*;
use husky_opn_syntax::Bracket;
use husky_print_utils::p;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprEnvironment {
    None,
    WithinBracket(Bracket),
}

impl Default for ExprEnvironment {
    fn default() -> Self {
        ExprEnvironment::None
    }
}

#[derive(Debug, Default)]
pub(super) struct ExprEnvironmentPlace(ExprEnvironment);
impl ExprEnvironmentPlace {
    pub(super) fn set(&mut self, env: ExprEnvironment) {
        if self.0 != ExprEnvironment::None {
            p!(self.0)
        }
        assert!(self.0 == ExprEnvironment::None);
        self.0 = env
    }

    pub(super) fn unset(&mut self) {
        self.0 = Default::default()
    }
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn env(&self) -> ExprEnvironment {
        self.env.0
    }
}
