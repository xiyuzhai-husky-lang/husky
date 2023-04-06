use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Contract {
    Pure,
    Move,
    Mut,
}

impl From<PatternModifier> for Contract {
    fn from(modifier: PatternModifier) -> Self {
        match modifier {
            PatternModifier::None => Contract::Pure,
            PatternModifier::Mut => Contract::Mut,
        }
    }
}
