mod substitution;

pub use self::substitution::*;

use crate::*;

pub trait DecTermRewrite: Sized {
    fn substitute(&self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self;
}

pub trait DecTermRewriteCopy: Copy {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self;
}

impl<T> DecTermRewrite for T
where
    T: DecTermRewriteCopy,
{
    fn substitute(&self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self {
        self.substitute_copy(db, substitution)
    }
}

impl DecTermRewriteCopy for DecTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self {
        match self {
            DecTerm::Rune(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            DecTerm::Symbol(_)
            | DecTerm::Literal(_)
            | DecTerm::EntityPath(_)
            | DecTerm::Category(_)
            | DecTerm::Universe(_)
            | DecTerm::LeashOrBitNot(_) => self,
            DecTerm::Curry(term) => term.substitute_copy(db, substitution).into(),
            DecTerm::Abstraction(term) => term.substitute_copy(db, substitution).into(),
            DecTerm::Application(term) => term.substitute_copy(db, substitution).into(),
            DecTerm::ApplicationOrRitchieCall(_term) => todo!(),
            DecTerm::AssociatedItem(term) => term.substitute_copy(db, substitution).into(),
            DecTerm::TypeAsTraitItem(term) => term.substitute_copy(db, substitution).into(),
            DecTerm::TraitConstraint(term) => term.substitute_copy(db, substitution).into(),
            DecTerm::Ritchie(_) => todo!(),
            DecTerm::List(_) => todo!(),
            DecTerm::Wrapper(_) => todo!(),
        }
    }
}
