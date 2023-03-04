mod substitution;

pub use substitution::*;

use crate::*;

pub trait ValidTermRewrite: Sized {
    fn substitute(&self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self;
}

pub trait ValidTermRewriteCopy: Copy {
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self;
}

impl<T> ValidTermRewrite for T
where
    T: ValidTermRewriteCopy,
{
    fn substitute(&self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        self.substitute_copy(db, substituation)
    }
}

impl ValidTermRewriteCopy for ValidTerm {
    fn substitute_copy(self, db: &dyn ValidTermDb, substitution: &ValidTermSubstitution) -> Self {
        match self {
            ValidTerm::Symbol(symbol) => match symbol == substitution.src() {
                true => substitution.dst(),
                false => self,
            },
            ValidTerm::Literal(_)
            | ValidTerm::EntityPath(_)
            | ValidTerm::Category(_)
            | ValidTerm::Universe(_) => self,
            ValidTerm::Curry(valid_term) => valid_term.substitute_copy(db, substitution).into(),
            ValidTerm::Abstraction(valid_term) => {
                valid_term.substitute_copy(db, substitution).into()
            }
            ValidTerm::Application(valid_term) => {
                valid_term.substitute_copy(db, substitution).into()
            }
            ValidTerm::Subentity(valid_term) => valid_term.substitute_copy(db, substitution).into(),
            ValidTerm::AsTraitSubentity(valid_term) => {
                valid_term.substitute_copy(db, substitution).into()
            }
            ValidTerm::TraitConstraint(valid_term) => {
                valid_term.substitute_copy(db, substitution).into()
            }
            ValidTerm::Ritchie(_) => todo!(),
        }
    }
}
