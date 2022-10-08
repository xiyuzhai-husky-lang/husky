use crate::*;
use husky_word::IdentPairDict;

pub trait AskDecl {
    fn ask_namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceDecl {
    members: IdentPairDict<Term>,
}
