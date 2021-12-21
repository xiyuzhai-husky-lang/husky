use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExprError {
    range: TextRange,
    rule_broken: ExprRule,
}

impl ExprError {
    pub fn new(range: TextRange, rule_broken: ExprRule) -> ExprError {
        Self { range, rule_broken }
    }
}

impl From<&atom::AtomError> for ExprError {
    fn from(_: &atom::AtomError) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExprRule {
    ScopeShouldBeCalled,
    BracketsShouldMatch,
    BracketedExprCommaListShouldBeEitherCalledOrIndexed,
}
