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
            Term::Variable(ident) => match substitution {
                TermSubstitution::Variable { src, dst } if ident == *src => *dst,
                _ => self,
            },
            Term::Lifetime(ident) => match substitution {
                TermSubstitution::Lifetime { src, dst } if ident == *src => Term::Lifetime(*dst),
                _ => self,
            },
            Term::Binding(ident) => match substitution {
                TermSubstitution::Binding { src, dst } if ident == *src => Term::Binding(*dst),
                _ => self,
            },
            Term::Literal(_) | Term::Entity(_) | Term::Category(_) | Term::Universe(_) => todo!(),
            Term::Curry(term) => term.substitute_copy(db, substitution).into(),
            Term::Abstraction(term) => term.substitute_copy(db, substitution).into(),
            Term::Application(term) => term.substitute_copy(db, substitution).into(),
            Term::Subentity(term) => term.substitute_copy(db, substitution).into(),
            Term::AsTraitSubentity(term) => term.substitute_copy(db, substitution).into(),
            Term::TraitConstraint(term) => term.substitute_copy(db, substitution).into(),
            Term::Jordan(_) => todo!(),
        }
    }
}
