use std::sync::Arc;

use crate::*;

#[salsa::query_group(TermQueryStorage)]
pub trait TermQuery: InternTerm + AskDecl {
    fn term_menu(&self) -> Arc<TermMenu>;
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
}

fn namespace_decl(db: &dyn TermQuery, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
    db.ask_namespace_decl(namespace)
}
