mod substitution;

pub use self::substitution::*;

use crate::*;

pub trait DeclarativeTermRewrite: Sized {
    fn substitute(
        &self,
        db: &dyn DeclarativeTermDb,
        substituation: &DeclarativeTermSubstitution,
    ) -> Self;
}

pub trait DeclarativeTermRewriteCopy: Copy {
    fn substitute(
        self,
        db: &dyn DeclarativeTermDb,
        substituation: &DeclarativeTermSubstitution,
    ) -> Self;
}

impl<T> DeclarativeTermRewrite for T
where
    T: DeclarativeTermRewriteCopy,
{
    fn substitute(
        &self,
        db: &dyn DeclarativeTermDb,
        substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        self.substitute(db, substituation)
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTerm {
    fn substitute(
        self,
        db: &dyn DeclarativeTermDb,
        substitution: &DeclarativeTermSubstitution,
    ) -> Self {
        match self {
            DeclarativeTerm::Hole(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            DeclarativeTerm::Symbol(_)
            | DeclarativeTerm::Literal(_)
            | DeclarativeTerm::EntityPath(_)
            | DeclarativeTerm::Category(_)
            | DeclarativeTerm::Universe(_)
            | DeclarativeTerm::LeashOrBitNot(_) => self,
            DeclarativeTerm::Curry(term) => term.substitute(db, substitution).into(),
            DeclarativeTerm::Abstraction(term) => term.substitute(db, substitution).into(),
            DeclarativeTerm::ExplicitApplication(term) => term.substitute(db, substitution).into(),
            DeclarativeTerm::ExplicitApplicationOrRitchieCall(_term) => todo!(),
            DeclarativeTerm::Subentity(term) => term.substitute(db, substitution).into(),
            DeclarativeTerm::AsTraitSubentity(term) => term.substitute(db, substitution).into(),
            DeclarativeTerm::TraitConstraint(term) => term.substitute(db, substitution).into(),
            DeclarativeTerm::Ritchie(_) => todo!(),
            DeclarativeTerm::List(_) => todo!(),
        }
    }
}
