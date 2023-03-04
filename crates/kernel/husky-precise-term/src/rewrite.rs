mod substitution;

pub use substitution::*;

use crate::*;

pub trait PreciseTermRewrite: Sized {
    fn substitute(&self, db: &dyn PreciseTermDb, substituation: &PreciseTermSubstitution) -> Self;
}

pub trait PreciseTermRewriteCopy: Copy {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self;
}

impl<T> PreciseTermRewrite for T
where
    T: PreciseTermRewriteCopy,
{
    fn substitute(&self, db: &dyn PreciseTermDb, substituation: &PreciseTermSubstitution) -> Self {
        self.substitute_copy(db, substituation)
    }
}

impl PreciseTermRewriteCopy for PreciseTerm {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substitution: &PreciseTermSubstitution,
    ) -> Self {
        match self {
            PreciseTerm::Symbol(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            PreciseTerm::Literal(_)
            | PreciseTerm::EntityPath(_)
            | PreciseTerm::Category(_)
            | PreciseTerm::Universe(_) => self,
            PreciseTerm::Curry(precise_term) => {
                precise_term.substitute_copy(db, substitution).into()
            }
            PreciseTerm::Abstraction(precise_term) => {
                precise_term.substitute_copy(db, substitution).into()
            }
            PreciseTerm::Application(precise_term) => {
                precise_term.substitute_copy(db, substitution).into()
            }
            PreciseTerm::Subentity(precise_term) => {
                precise_term.substitute_copy(db, substitution).into()
            }
            PreciseTerm::AsTraitSubentity(precise_term) => {
                precise_term.substitute_copy(db, substitution).into()
            }
            PreciseTerm::TraitConstraint(precise_term) => {
                precise_term.substitute_copy(db, substitution).into()
            }
            PreciseTerm::Ritchie(_) => todo!(),
        }
    }
}
