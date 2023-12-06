use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    Pure,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
    At,
}

impl Contract {
    pub fn new<TG>(ephem_symbol_modifier_token_group: Option<TG>) -> Self
    where
        TG: Into<Contract>,
    {
        match ephem_symbol_modifier_token_group {
            Some(t) => t.into(),
            None => Contract::Pure,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Contract::Pure => "",
            Contract::Move => "move ",
            Contract::Borrow => "borrow",
            Contract::BorrowMut => "borrow mut",
            Contract::Const => "const",
            Contract::Leash => todo!(),
            Contract::At => "@",
        }
    }
}

impl From<SymbolModifier> for Contract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::Pure => Contract::Pure,
            SymbolModifier::Mut => Contract::Move,
            SymbolModifier::Ref => Contract::Borrow,
            SymbolModifier::RefMut => Contract::BorrowMut,
            SymbolModifier::Const => Contract::Const,
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
        }
    }
}
