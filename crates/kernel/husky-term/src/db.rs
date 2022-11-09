use std::sync::Arc;

use husky_entity_path::InternEntityPath;
use husky_word::InternWord;

use crate::*;

#[salsa::query_group(TermDbStorage)]
pub trait TermDb: InternTerm + InternEntityPath + InternWord + AskDecl {
    fn term_menu(&self) -> Arc<TermMenu>;
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
    fn ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl>;
    fn decl(&self, entity_path: EntityPathItd) -> TermResultArc<Decl>;
}

fn term_menu(db: &dyn TermDb) -> Arc<TermMenu> {
    TermMenu::new(db)
}

fn namespace_decl(db: &dyn TermDb, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
    db.ask_namespace_decl(namespace)
}

fn ty_decl(db: &dyn TermDb, ty: Ty) -> TermResultArc<TyDecl> {
    db.ask_ty_decl(ty)
}

fn decl(db: &dyn TermDb, entity_path: EntityPathItd) -> TermResultArc<Decl> {
    db.ask_decl(entity_path)
}
