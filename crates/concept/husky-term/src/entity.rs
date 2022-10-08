mod intern;
mod namespace;

pub use intern::*;
pub use namespace::*;

use husky_word::Identifier;
use optional::Optioned;

use crate::{TermQuery, Ty};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermEntityPath {
    opt_parent: Optioned<TermEntityPathPtr>,
    name: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermEntity {
    namespace: TermEntityPath,
    ty: Ty,
}

impl TermEntity {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub(crate) fn i32(db: &dyn TermQuery) -> Self {
        todo!()
    }
}
