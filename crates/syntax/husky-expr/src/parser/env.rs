use super::*;
use husky_opn_syntax::Bracket;
use husky_print_utils::p;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExprEnvironment {
    eval: ExprEvalEnvironment,
    bra: Option<Bracket>,
}

impl ExprEnvironment {
    pub fn new(eval: ExprEvalEnvironment, bra: impl Into<Option<Bracket>>) -> Self {
        Self {
            eval,
            bra: bra.into(),
        }
    }

    pub fn eval(&self) -> ExprEvalEnvironment {
        self.eval
    }

    pub fn bra(&self) -> Option<Bracket> {
        self.bra
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprEvalEnvironment {
    Runtime = 1,
    Comptime = 2,
}

pub(super) struct ExprEnvironmentPlace {
    base: ExprEnvironment,
    /// when None, means current is base
    current: Option<ExprEnvironment>,
}
impl ExprEnvironmentPlace {
    pub(super) fn set(&mut self, env: ExprEnvironment) {
        if self.current != None {
            p!(self.current)
        }
        assert!(self.current == None);
        self.current = Some(env)
    }

    pub(super) fn unset(&mut self) {
        self.current = None
    }

    pub(super) fn current(&self) -> ExprEnvironment {
        self.current.unwrap_or(self.base)
    }

    pub(crate) fn new(base: ExprEnvironment) -> ExprEnvironmentPlace {
        ExprEnvironmentPlace {
            base,
            current: None,
        }
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn env(&self) -> ExprEnvironment {
        self.env.current()
    }
}
