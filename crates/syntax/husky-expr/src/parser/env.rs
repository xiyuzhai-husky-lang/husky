use super::*;
use husky_opn_syntax::Bracket;
use husky_print_utils::p;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ExprEnvironment {
    bra: Option<Bracket>,
}

impl ExprEnvironment {
    pub fn new(bra: impl Into<Option<Bracket>>) -> Self {
        Self { bra: bra.into() }
    }

    pub(super) fn set(&mut self, env_bra: Bracket) {
        assert!(self.bra == None);
        self.bra = Some(env_bra)
    }

    pub(super) fn unset(&mut self) {
        self.bra = None
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn env_bra(&self) -> Option<Bracket> {
        self.env.bra
    }
}
