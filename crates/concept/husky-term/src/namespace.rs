mod intern;

use husky_word::Identifier;
pub use intern::*;
use optional::Optioned;

use crate::{TermQuery, Ty};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Namespace {
    opt_parent: Optioned<NamespacePtr>,
    name: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermNamespace {
    namespace: Namespace,
    ty: Ty,
}

impl TermNamespace {
    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub(crate) fn i32(db: &dyn TermQuery) -> Self {
        todo!()
    }
}
