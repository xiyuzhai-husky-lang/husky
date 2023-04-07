#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(trait_upcasting)]
mod engine;
mod expectation;
mod local_term;
mod pattern;
mod progress;
mod region;
mod utils;

use husky_print_utils::p;

pub use self::engine::*;
pub use self::expectation::*;
pub use self::local_term::*;
pub use self::pattern::*;
pub use self::progress::*;
pub use self::region::*;

use either::*;
use husky_entity_path::*;
use husky_expr::*;
use husky_term::*;
use husky_term_prelude::*;
use salsa::DebugWithDb as _;
use smallvec::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TermDb, jar = TermJar)]
#[enum_class::from_variants]
pub enum LocalTerm {
    Term(Term),
    Unresolved(LocalTermIdx),
    PlaceType(PlaceTypeIdx),
}
#[test]
fn t() {
    println!("{}", std::mem::size_of::<LocalTerm>())
}

impl LocalTerm {
    pub fn unravel_borrow(self, db: &dyn TermDb, unresolved_terms: &UnresolvedTerms) -> Self {
        match self.pattern_inner(db, unresolved_terms) {
            LocalTermPattern::TypeOntology {
                refined_path: Right(PreludeTypePath::Borrow(path)),
                argument_tys: arguments,
                ..
            } => match path {
                PreludeBorrowTypePath::Ref | PreludeBorrowTypePath::RefMut => {
                    assert_eq!(arguments.len(), 2);
                    todo!()
                }
                PreludeBorrowTypePath::Leash => {
                    assert_eq!(arguments.len(), 1);
                    arguments[0]
                }
            },
            _ => self,
        }
    }

    fn resolved(self) -> Option<Term> {
        match self {
            LocalTerm::Term(term) => Some(term),
            LocalTerm::Unresolved(_) => None,
            LocalTerm::PlaceType(_) => todo!(),
        }
    }

    pub(crate) fn resolve_progress(
        self,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermResolveResultRef<Self> {
        match self {
            LocalTerm::Term(term) => Ok(term.into()),
            LocalTerm::Unresolved(idx) => idx.resolve_progress(unresolved_terms),
            LocalTerm::PlaceType(_) => todo!(),
        }
    }
}

impl From<TermLiteral> for LocalTerm {
    fn from(value: TermLiteral) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermSymbol> for LocalTerm {
    fn from(value: TermSymbol) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermPlaceholder> for LocalTerm {
    fn from(value: TermPlaceholder) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermEntityPath> for LocalTerm {
    fn from(value: TermEntityPath) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermCategory> for LocalTerm {
    fn from(value: TermCategory) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermUniverse> for LocalTerm {
    fn from(value: TermUniverse) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermCurry> for LocalTerm {
    fn from(value: TermCurry) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermRitchie> for LocalTerm {
    fn from(value: TermRitchie) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermAbstraction> for LocalTerm {
    fn from(value: TermAbstraction) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermApplication> for LocalTerm {
    fn from(value: TermApplication) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermSubentity> for LocalTerm {
    fn from(value: TermSubentity) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermAsTraitSubentity> for LocalTerm {
    fn from(value: TermAsTraitSubentity) -> Self {
        LocalTerm::Term(value.into())
    }
}

impl From<TermTraitConstraint> for LocalTerm {
    fn from(value: TermTraitConstraint) -> Self {
        LocalTerm::Term(value.into())
    }
}
