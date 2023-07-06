use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    None,
    Move,
    BorrowMut,
    Const,
}

impl Contract {
    pub fn new<SymbolModifierKeywordGroup>(
        keyword_group: Option<SymbolModifierKeywordGroup>,
    ) -> Self
    where
        SymbolModifierKeywordGroup: Into<Contract>,
    {
        match keyword_group {
            Some(t) => t.into(),
            None => Contract::None,
        }
    }
}

impl From<SymbolModifier> for Contract {
    fn from(modifier: SymbolModifier) -> Self {
        match modifier {
            SymbolModifier::None => Contract::None,
            SymbolModifier::Mut => Contract::Move,
            SymbolModifier::RefMut => Contract::BorrowMut,
            SymbolModifier::Const => Contract::Const,
        }
    }
}
