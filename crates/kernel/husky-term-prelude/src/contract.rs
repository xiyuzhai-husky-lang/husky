use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermContract {
    Pure,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
    At,
}

impl TermContract {
    pub fn new<TG>(ephem_symbol_modifier_token_group: Option<TG>) -> Self
    where
        TG: Into<TermContract>,
    {
        match ephem_symbol_modifier_token_group {
            Some(t) => t.into(),
            None => TermContract::Pure,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            TermContract::Pure => "",
            TermContract::Move => "move ",
            TermContract::Borrow => "borrow",
            TermContract::BorrowMut => "borrow mut",
            TermContract::Const => "const",
            TermContract::Leash => todo!(),
            TermContract::At => "@",
        }
    }
}

impl From<SymbolModifier> for TermContract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::Pure => TermContract::Pure,
            SymbolModifier::Owned | SymbolModifier::Mut => TermContract::Move,
            SymbolModifier::Ref => TermContract::Borrow,
            SymbolModifier::RefMut => TermContract::BorrowMut,
            SymbolModifier::Const => TermContract::Const,
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
        }
    }
}
