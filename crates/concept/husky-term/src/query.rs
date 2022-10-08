use std::sync::Arc;

use crate::*;

#[salsa::query_group(TermQueryStorage)]
pub trait TermQuery: InternTerm + AskDecl {
    fn term_menu(&self) -> Arc<TermMenu>;
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu {
    i32: TermPtr,
    i64: TermPtr,
}

fn term_menu(db: &dyn TermQuery) -> Arc<TermMenu> {
    Arc::new(TermMenu {
        i32: TermEntity::i32(db),
        i64: TermEntity::i64(db),
    })
}

fn namespace_decl(db: &dyn TermQuery, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
    db.ask_namespace_decl(namespace)
}
