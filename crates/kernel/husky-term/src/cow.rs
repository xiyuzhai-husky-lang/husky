// use std::ops::Deref;

// use crate::*;

// pub(crate) enum TermCow {
//     Owned(TermData),
//     Ptr(Term),
// }

// impl std::fmt::Debug for TermCow {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.deref().fmt(f)
//     }
// }

// impl From<TermData> for TermCow {
//     fn from(term: TermData) -> Self {
//         Self::Owned(term)
//     }
// }

// impl From<Term> for TermCow {
//     fn from(ptr: Term) -> Self {
//         Self::Ptr(ptr)
//     }
// }

// impl std::ops::Deref for TermCow {
//     type Target = TermData;

//     fn deref(&self) -> &Self::Target {
//         match self {
//             TermCow::Owned(ref term) => term,
//             TermCow::Ptr(ptr) => &*ptr,
//         }
//     }
// }
