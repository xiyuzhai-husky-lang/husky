use husky_word::Identifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermVariable {
    opt_namespace: Optioned<NamespacePtr>,
    name: Identifier,
}

impl TermVariable {
    pub fn ty(&self) -> Ty {
        todo!()
    }

    pub fn universe(&self) -> Universe {
        todo!()
    }
}
