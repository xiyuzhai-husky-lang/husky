mod error;

use std::ops::Deref;

use husky_print_utils::p;

use crate::{
    cow::TermCow,
    error::{TermError, TermResult},
    intern::new_term_interner,
    Term, TermCurry, TermInterner, TermPtr, Universe,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ty(TermPtr);

impl std::ops::Deref for Ty {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl Ty {
    pub(crate) fn new(term: TermPtr) -> TermResult<Self> {
        match *term {
            Term::Type(_) | Term::Curry(TermCurry { .. }) => (),
            Term::Abstraction(_) => return Err(TermError::TermIsNotTy),
            Term::Variable(_) | Term::Application(_) => {
                if !matches!(&*term.ty_term(), &Term::Type(_)) {
                    return Err(TermError::TermIsNotTy);
                }
            }
        }
        Ok(Self(term))
    }

    pub fn term(self) -> TermPtr {
        self.0
    }
}

impl Term {
    pub(crate) fn ty_term(&self) -> TermCow {
        match self {
            Term::Type(u) => Term::Type(u.next().expect("todo")).into(),
            Term::Curry(c) => c.ty().term().into(),
            Term::Variable(v) => v.ty().term().into(),
            Term::Abstraction(abs) => abs.ty().term().into(),
            Term::Application(app) => app.ty().term().into(),
        }
    }

    // pub(crate) fn universe(&self) -> Universe {
    //     match self {
    //         Term::Type(u) => u.next().expect("todo"),
    //         Term::Curry(TermCurry { from, to }) => {
    //             from.universe().max(to.universe()).next().expect("todo")
    //         }
    //         Term::Variable(v) => v.universe(),
    //         Term::Abstraction(abs) => abs.universe(),
    //         Term::Application(app) => app.universe(),
    //     }
    // }
}

#[test]
fn test_ty_term() {
    let term_interner = new_term_interner();
    let term = term_interner.intern(Term::Type(Universe::zero()));
    assert_eq!(
        Term::Type(Universe::zero()).ty_term().deref(),
        &Term::Type(Universe::zero().next().unwrap())
    );
}
