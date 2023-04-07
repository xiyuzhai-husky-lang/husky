use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PatternContract {
    Pure,
    Move,
    Mut,
    ConstExpr,
    StaticExpr,
}

impl From<SymbolModifier> for PatternContract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::None => PatternContract::Pure,
            SymbolModifier::Mut => PatternContract::Mut,
            SymbolModifier::ConstExpr => PatternContract::ConstExpr,
            SymbolModifier::StaticExpr => PatternContract::StaticExpr,
        }
    }
}
