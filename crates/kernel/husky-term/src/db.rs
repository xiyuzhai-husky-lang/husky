use std::sync::Arc;

use husky_entity_path::InternEntityPath;
use husky_word::InternWord;

use crate::*;

#[salsa::query_group(TermDbStorage)]
pub trait TermDb: InternTerm + AskDecl + InternEntityPath + InternWord {
    fn term_menu(&self) -> Arc<TermMenu>;
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
    fn ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl>;
}

fn namespace_decl(db: &dyn TermDb, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
    db.ask_namespace_decl(namespace)
}

fn ty_decl(db: &dyn TermDb, ty: Ty) -> TermResultArc<TyDecl> {
    db.ask_ty_decl(ty)
}
