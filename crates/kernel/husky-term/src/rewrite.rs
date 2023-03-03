mod substitution;

pub use substitution::*;

use crate::*;

pub trait TermRewrite: Sized {
    fn substitute(&self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self;
}

pub trait TermRewriteCopy: Copy {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self;
}

impl<T> TermRewrite for T
where
    T: TermRewriteCopy,
{
    fn substitute(&self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        self.substitute_copy(db, substituation)
    }
}

impl TermRewriteCopy for Term {
    fn substitute_copy(self, db: &dyn TermDb, substitution: &TermSubstitution) -> Self {
        match self {
            Term::Symbol(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            Term::Literal(_) | Term::Entity(_) | Term::Category(_) | Term::Universe(_) => self,
            Term::Curry(term) => term.substitute_copy(db, substitution).into(),
            Term::Abstraction(term) => term.substitute_copy(db, substitution).into(),
            Term::Application(term) => term.substitute_copy(db, substitution).into(),
            Term::Subentity(term) => term.substitute_copy(db, substitution).into(),
            Term::AsTraitSubentity(term) => term.substitute_copy(db, substitution).into(),
            Term::TraitConstraint(term) => term.substitute_copy(db, substitution).into(),
            Term::Ritchie(_) => todo!(),
            Term::Composition(_) => todo!(),
        }
    }
}
