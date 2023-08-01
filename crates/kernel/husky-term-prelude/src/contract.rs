use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    None,
    Move,
    Borrow,
    BorrowMut,
    Const,
    Leash,
}

impl Contract {
    pub fn new<TG>(ephem_symbol_modifier_token_group: Option<TG>) -> Self
    where
        TG: Into<Contract>,
    {
        match ephem_symbol_modifier_token_group {
            Some(t) => t.into(),
            None => Contract::None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Contract::None => "",
            Contract::Move => "move ",
            Contract::Borrow => "borrow",
            Contract::BorrowMut => "borrow mut",
            Contract::Const => "const",
            Contract::Leash => todo!(),
        }
    }
}

impl From<EphemSymbolModifier> for Contract {
    fn from(modifier: EphemSymbolModifier) -> Self {
        match modifier {
            EphemSymbolModifier::None => Contract::None,
            EphemSymbolModifier::Mut => Contract::Move,
            EphemSymbolModifier::RefMut => Contract::BorrowMut,
            EphemSymbolModifier::Const => Contract::Const,
            EphemSymbolModifier::Ambersand(_) => todo!(),
            EphemSymbolModifier::AmbersandMut(_) => todo!(),
            EphemSymbolModifier::Le => todo!(),
            EphemSymbolModifier::Tilde => todo!(),
        }
    }
}
