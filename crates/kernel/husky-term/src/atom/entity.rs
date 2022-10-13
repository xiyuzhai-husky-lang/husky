mod namespace;
mod root;

pub use namespace::*;

use husky_entity_path::EntityPathPtr;

use husky_word::{Identifier, RootBuiltinIdentifier};
use optional::Optioned;

use crate::{Term, TermDb, Ty};

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct TermEntity {
//     path: EntityPathPtr,
//     ty: Ty,
// }

// impl TermEntity {
//     pub fn ty(&self) -> Ty {
//         self.ty
//     }

//     pub fn path(&self) -> EntityPathPtr {
//         self.path
//     }
// }

// impl Into<Term> for TermEntity {
//     fn into(self) -> Term {
//         Term::AtomTermTerm::Entity(self)
//     }
// }
