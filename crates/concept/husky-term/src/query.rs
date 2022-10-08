use crate::*;

#[salsa::query_group(TermQueryStorage)]
pub trait TermQuery: InternTerm + AskDecl {
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
}

fn namespace_decl(db: &dyn TermQuery, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
    db.ask_namespace_decl(namespace)
}
