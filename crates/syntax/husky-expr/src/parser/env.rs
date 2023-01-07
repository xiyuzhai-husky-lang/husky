use super::*;
use husky_opn_syntax::Bracket;
use husky_print_utils::p;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprParseEnvironment {
    None,
    WithinBracket(Bracket),
}

impl Default for ExprParseEnvironment {
    fn default() -> Self {
        ExprParseEnvironment::None
    }
}

#[derive(Debug, Default)]
pub(super) struct ExprParseEnvironmentPlace(ExprParseEnvironment);
impl ExprParseEnvironmentPlace {
    pub(super) fn set(&mut self, env: ExprParseEnvironment) {
        if self.0 != ExprParseEnvironment::None {
            p!(self.0)
        }
        assert!(self.0 == ExprParseEnvironment::None);
        self.0 = env
    }

    pub(super) fn unset(&mut self) {
        self.0 = Default::default()
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn env(&self) -> ExprParseEnvironment {
        self.env.0
    }
}
