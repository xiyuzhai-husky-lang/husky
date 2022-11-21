use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_identifier::IdentifierDb;
use salsa::DbWithJar;

pub trait TermDb: DbWithJar<TermJar> {
    fn term_menu(&self) -> Arc<TermMenu>;
    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl>;
    fn ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl>;
    fn decl(&self, entity_path: EntityPath) -> TermResultArc<Decl>;
}

impl<T> TermDb for T
where
    T: DbWithJar<TermJar>,
{
    fn term_menu(&self) -> Arc<TermMenu> {
        todo!()
    }

    fn namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
        todo!()
    }

    fn ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn decl(&self, entity_path: EntityPath) -> TermResultArc<Decl> {
        todo!()
    }
}

fn term_menu(db: &dyn TermDb) -> Arc<TermMenu> {
    TermMenu::new(db)
}

// fn namespace_decl(db: &dyn TermDb, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
//     db.ask_namespace_decl(namespace)
// }

// fn ty_decl(db: &dyn TermDb, ty: Ty) -> TermResultArc<TyDecl> {
//     db.ask_ty_decl(ty)
// }

// fn decl(db: &dyn TermDb, entity_path: EntityPath) -> TermResultArc<Decl> {
//     db.ask_decl(entity_path)
// }
