

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TermNamespace(TermItd);

// impl Noned for TermNamespace {
//     fn is_none(&self) -> bool {
//         self.0.is_none()
//     }

//     fn get_none() -> Self {
//         Self(TermItd::get_none())
//     }
// }

// impl OptEq for TermNamespace {
//     fn opt_eq(&self, other: &Self) -> bool {
//         self.0.opt_eq(&other.0)
//     }
// }
