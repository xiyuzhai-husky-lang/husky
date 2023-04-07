use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PatternContract {
    Pure,
    Move,
    Mut,
    Const,
}

impl From<SymbolModifier> for PatternContract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::Pure => PatternContract::Pure,
            SymbolModifier::Mut => PatternContract::Mut,
            SymbolModifier::Const => PatternContract::Const,
        }
    }
}
