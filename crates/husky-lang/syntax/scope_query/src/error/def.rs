use text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScopeDefError {
    pub range: TextRange,
    pub rule_broken: ScopeDefRule,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScopeDefRule {
    TokenGroupSizeAtLeastTwo,
    FirstTokenShouldBeKeyword,
    NonMainSecondTokenShouldBeIdentifier,
    BuiltinIdentifierOrElideAreReserved,
}

macro_rules! build_error_code_gen {
($grammar_failed: expr, $($item:ident), *) => {{
    match $grammar_failed {
        $(ScopeDefRule::$item => concat!("rule broken: ScopeDefRule::", stringify!($item))),*
    }
}};
}

impl ScopeDefError {
    pub fn code(&self) -> &'static str {
        build_error_code_gen!(
            self.rule_broken,
            TokenGroupSizeAtLeastTwo,
            FirstTokenShouldBeKeyword,
            NonMainSecondTokenShouldBeIdentifier,
            BuiltinIdentifierOrElideAreReserved
        )
    }
}
