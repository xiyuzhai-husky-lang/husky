use dev_utils::DevSource;
use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EntityDefnError {
    pub range: TextRange,
    pub rule_broken: ScopeDefRule,
    pub dev_src: DevSource,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeDefRule {
    TokenGroupSizeAtLeastTwo,
    FirstTokenShouldBeKeyword,
    SecondTokenShouldBeIdentifier,
    BuiltinIdentifierAreReserved,
    ContextualIdentifierAreReserved,
}

impl EntityDefnError {
    pub fn message(&self) -> String {
        match self.rule_broken {
            ScopeDefRule::TokenGroupSizeAtLeastTwo => "token group size at least two",
            ScopeDefRule::FirstTokenShouldBeKeyword => "first token should be keyword",
            ScopeDefRule::SecondTokenShouldBeIdentifier => {
                "non main second token should be identifier"
            }
            ScopeDefRule::BuiltinIdentifierAreReserved => "builtin identifiers are reserved",
            ScopeDefRule::ContextualIdentifierAreReserved => "contextual identifiers are reserved",
        }
        .into()
    }
}
