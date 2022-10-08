use crate::*;
use husky_word::IdentPairDict;

pub trait AskDecl {
    fn ask_namespace_decl(&self) -> TermResultArc<NamespaceDecl>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct NamespaceDecl {
    members: IdentPairDict<Term>,
}
