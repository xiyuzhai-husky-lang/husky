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
    NonMainSecondTokenShouldBeIdentifier,
    BuiltinIdentifierAreReserved,
    ContextualIdentifierAreReserved,
}

// macro_rules! build_error_code_gen {
// ($grammar_failed: expr, $($item:ident), *) => {{
//     match $grammar_failed {
//         $(ScopeDefRule::$item => concat!("rule broken: ScopeDefRule::", stringify!($item))),*
//     }
// }};
// }

impl EntityDefnError {
    pub fn message(&self) -> String {
        match self.rule_broken {
            ScopeDefRule::TokenGroupSizeAtLeastTwo => todo!(),
            ScopeDefRule::FirstTokenShouldBeKeyword => {
                "Syntax Error: first token should be keyword"
            }
            ScopeDefRule::NonMainSecondTokenShouldBeIdentifier => todo!(),
            ScopeDefRule::BuiltinIdentifierAreReserved => todo!(),
            ScopeDefRule::ContextualIdentifierAreReserved => todo!(),
        }
        .into()
    }
}
