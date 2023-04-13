use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    Pure,
    Move,
    BorrowMut,
    Const,
}

impl From<SymbolModifier> for Contract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::Pure => Contract::Pure,
            SymbolModifier::Mut => todo!(),
            SymbolModifier::RefMut => todo!(),
            SymbolModifier::Const => Contract::Const,
        }
    }
}
