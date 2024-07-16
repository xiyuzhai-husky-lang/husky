use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    Pure,
    Move,
    Borrow,
    BorrowMut,
    Compterm,
    Leash,
    At,
}

impl Contract {
    pub fn new<TG>(ephem_symbol_modifier_token_verse: Option<TG>) -> Self
    where
        TG: Into<Contract>,
    {
        match ephem_symbol_modifier_token_verse {
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
            Contract::Compterm => "const",
            Contract::Leash => todo!(),
            Contract::At => "@",
        }
    }

    pub fn merge(contracts: impl IntoIterator<Item = Self>) -> Self {
        let mut contracts = contracts.into_iter();
        let Some(mut contract) = contracts.next() else {
            return Contract::Pure;
        };
        for next in contracts {
            contract *= next
        }
        contract
    }
}

impl std::ops::MulAssign<Self> for Contract {
    fn mul_assign(&mut self, rhs: Self) {
        *self = match (*self, rhs) {
            (slf, rhs) if slf == rhs => return,
            (Contract::At, Contract::Pure) => Contract::Pure,
            (_, Contract::At | Contract::Pure) => return,
            (Contract::At | Contract::Pure, rhs) => rhs,
            (Contract::Move, Contract::Move) => todo!(),
            (Contract::Move, Contract::Borrow) => todo!(),
            (Contract::Move, Contract::BorrowMut) => todo!(),
            (Contract::Move, Contract::Compterm) => todo!(),
            (Contract::Move, Contract::Leash) => todo!(),
            (Contract::Borrow, Contract::Move) => todo!(),
            (Contract::Borrow, Contract::Borrow) => todo!(),
            (Contract::Borrow, Contract::BorrowMut) => todo!(),
            (Contract::Borrow, Contract::Compterm) => todo!(),
            (Contract::Borrow, Contract::Leash) => todo!(),
            (Contract::BorrowMut, Contract::Move) => todo!(),
            (Contract::BorrowMut, Contract::Borrow) => todo!(),
            (Contract::BorrowMut, Contract::Compterm) => todo!(),
            (Contract::BorrowMut, Contract::Leash) => todo!(),
            (Contract::Compterm, Contract::Move) => todo!(),
            (Contract::Compterm, Contract::Borrow) => todo!(),
            (Contract::Compterm, Contract::BorrowMut) => todo!(),
            (Contract::Compterm, Contract::Compterm) => todo!(),
            (Contract::Compterm, Contract::Leash) => todo!(),
            (Contract::Leash, Contract::Move) => todo!(),
            (Contract::Leash, Contract::Borrow) => todo!(),
            (Contract::Leash, Contract::BorrowMut) => todo!(),
            (Contract::Leash, Contract::Compterm) => todo!(),
            (Contract::BorrowMut, Contract::BorrowMut) => todo!(),
            (Contract::Leash, Contract::Leash) => todo!(),
        }
    }
}

impl From<VariableModifier> for Contract {
    fn from(modifier: VariableModifier) -> Self {
        match modifier {
            VariableModifier::Pure => Contract::Pure,
            VariableModifier::Owned | VariableModifier::Mut => Contract::Move,
            VariableModifier::Ref => Contract::Borrow,
            VariableModifier::RefMut => Contract::BorrowMut,
            VariableModifier::Compterm => Contract::Compterm,
            VariableModifier::Ambersand(_) => todo!(),
            VariableModifier::AmbersandMut(_) => todo!(),
            VariableModifier::Le => todo!(),
            VariableModifier::Tilde => todo!(),
            VariableModifier::At => todo!(),
        }
    }
}
