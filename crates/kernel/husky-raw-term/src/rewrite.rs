mod substitution;

pub use substitution::*;

use crate::*;

pub trait RawTermRewrite: Sized {
    fn substitute(&self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self;
}

pub trait RawTermRewriteCopy: Copy {
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self;
}

impl<T> RawTermRewrite for T
where
    T: RawTermRewriteCopy,
{
    fn substitute(&self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        self.substitute(db, substituation)
    }
}

impl RawTermRewriteCopy for RawTerm {
    fn substitute(self, db: &dyn RawTermDb, substitution: &RawTermSubstitution) -> Self {
        match self {
            RawTerm::Symbol(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            RawTerm::Literal(_)
            | RawTerm::EntityPath(_)
            | RawTerm::Category(_)
            | RawTerm::Universe(_)
            | RawTerm::LeashOrBitNot(_) => self,
            RawTerm::Curry(term) => term.substitute(db, substitution).into(),
            RawTerm::Abstraction(term) => term.substitute(db, substitution).into(),
            RawTerm::ExplicitApplication(term) => term.substitute(db, substitution).into(),
            RawTerm::ExplicitApplicationOrRitchieCall(term) => todo!(),
            RawTerm::Subentity(term) => term.substitute(db, substitution).into(),
            RawTerm::AsTraitSubentity(term) => term.substitute(db, substitution).into(),
            RawTerm::TraitConstraint(term) => term.substitute(db, substitution).into(),
            RawTerm::Ritchie(_) => todo!(),
            RawTerm::List(_) => todo!(),
        }
    }
}
