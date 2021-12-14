use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeDefError {
    pub range: TextRange,
    pub grammar_failed: ScopeDefGrammar,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeDefGrammar {
    TokenGroupSizeAtLeastTwo,
    FirstTokenShouldBeKeyword,
    SecondTokenShouldBeIdentifier,
}
