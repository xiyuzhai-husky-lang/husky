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
    pub fn new<TG>(ephem_symbol_modifier_token_verse: Option<TG>) -> Self
    where
        TG: Into<TermContract>,
    {
        match ephem_symbol_modifier_token_verse {
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

impl From<SvarModifier> for TermContract {
    fn from(modifier: SvarModifier) -> Self {
        match modifier {
            SvarModifier::Pure => TermContract::Pure,
            SvarModifier::Owned | SvarModifier::Mut => TermContract::Move,
            SvarModifier::Ref => TermContract::Borrow,
            SvarModifier::RefMut => TermContract::BorrowMut,
            SvarModifier::Const => TermContract::Const,
            SvarModifier::Ambersand(_) => todo!(),
            SvarModifier::AmbersandMut(_) => todo!(),
            SvarModifier::Le => todo!(),
            SvarModifier::Tilde => todo!(),
            SvarModifier::At => todo!(),
        }
    }
}
