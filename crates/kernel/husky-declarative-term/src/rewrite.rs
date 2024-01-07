mod substitution;

pub use self::substitution::*;

use crate::*;

pub trait DeclarativeTermRewrite: Sized {
    fn substitute(&self, db: &::salsa::Db, substituation: &DeclarativeTermSubstitution) -> Self;
}

pub trait DeclarativeTermRewriteCopy: Copy {
    fn substitute_copy(self, db: &::salsa::Db, substituation: &DeclarativeTermSubstitution)
        -> Self;
}

impl<T> DeclarativeTermRewrite for T
where
    T: DeclarativeTermRewriteCopy,
{
    fn substitute(&self, db: &::salsa::Db, substituation: &DeclarativeTermSubstitution) -> Self {
        self.substitute_copy(db, substituation)
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DeclarativeTermSubstitution) -> Self {
        match self {
            DeclarativeTerm::Rune(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            DeclarativeTerm::Symbol(_)
            | DeclarativeTerm::Literal(_)
            | DeclarativeTerm::EntityPath(_)
            | DeclarativeTerm::Category(_)
            | DeclarativeTerm::Universe(_)
            | DeclarativeTerm::LeashOrBitNot(_) => self,
            DeclarativeTerm::Curry(term) => term.substitute_copy(db, substitution).into(),
            DeclarativeTerm::Abstraction(term) => term.substitute_copy(db, substitution).into(),
            DeclarativeTerm::ExplicitApplication(term) => {
                term.substitute_copy(db, substitution).into()
            }
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_term) => todo!(),
            DeclarativeTerm::Subitem(term) => term.substitute_copy(db, substitution).into(),
            DeclarativeTerm::AsTraitSubitem(term) => term.substitute_copy(db, substitution).into(),
            DeclarativeTerm::TraitConstraint(term) => term.substitute_copy(db, substitution).into(),
            DeclarativeTerm::Ritchie(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
            DeclarativeTerm::Wrapper(_) => todo!(),
        }
    }
}
