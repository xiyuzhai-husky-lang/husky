use std::ops::Deref;

use crate::*;

pub(crate) enum TermCow {
    Owned(Term),
    Ptr(TermItd),
}

impl std::fmt::Debug for TermCow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl From<Term> for TermCow {
    fn from(term: Term) -> Self {
        Self::Owned(term)
    }
}

impl From<TermItd> for TermCow {
    fn from(ptr: TermItd) -> Self {
        Self::Ptr(ptr)
    }
}

impl std::ops::Deref for TermCow {
    type Target = Term;

    fn deref(&self) -> &Self::Target {
        match self {
            TermCow::Owned(ref term) => term,
            TermCow::Ptr(ptr) => &*ptr,
        }
    }
}
