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
    NonMainSecondTokenShouldBeIdentifier,
    TokenGroupOfSizeTwoShouldBeMain,
    GenericsShouldBeWellFormed,
}

macro_rules! build_error_code_gen {
    ($grammar_failed: expr, $($item:ident), *) => {{
        match $grammar_failed {
            $(ScopeDefGrammar::$item => concat!("grammar failed: ScopeDefGrammar::", stringify!($item))),*
        }
    }};
}

impl ScopeDefError {
    pub fn code(&self) -> &'static str {
        build_error_code_gen!(
            self.grammar_failed,
            TokenGroupSizeAtLeastTwo,
            FirstTokenShouldBeKeyword,
            NonMainSecondTokenShouldBeIdentifier,
            TokenGroupOfSizeTwoShouldBeMain,
            GenericsShouldBeWellFormed
        )
    }
}
