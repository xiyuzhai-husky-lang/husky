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
    Resolved(Term),
    Unresolved(UnresolvedTermIdx),
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
            LocalTerm::Resolved(term) => Some(term),
            LocalTerm::Unresolved(_) => None,
        }
    }

    pub(crate) fn resolve_progress(
        self,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermResolveResultRef<Self> {
        match self {
            LocalTerm::Resolved(term) => Ok(term.into()),
            LocalTerm::Unresolved(idx) => idx.resolve_progress(unresolved_terms),
        }
    }
}

impl From<TermLiteral> for LocalTerm {
    fn from(value: TermLiteral) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermSymbol> for LocalTerm {
    fn from(value: TermSymbol) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermPlaceholder> for LocalTerm {
    fn from(value: TermPlaceholder) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermEntityPath> for LocalTerm {
    fn from(value: TermEntityPath) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCategory> for LocalTerm {
    fn from(value: TermCategory) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermUniverse> for LocalTerm {
    fn from(value: TermUniverse) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCurry> for LocalTerm {
    fn from(value: TermCurry) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermRitchie> for LocalTerm {
    fn from(value: TermRitchie) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAbstraction> for LocalTerm {
    fn from(value: TermAbstraction) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermApplication> for LocalTerm {
    fn from(value: TermApplication) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermSubentity> for LocalTerm {
    fn from(value: TermSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAsTraitSubentity> for LocalTerm {
    fn from(value: TermAsTraitSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermTraitConstraint> for LocalTerm {
    fn from(value: TermTraitConstraint) -> Self {
        LocalTerm::Resolved(value.into())
    }
}
