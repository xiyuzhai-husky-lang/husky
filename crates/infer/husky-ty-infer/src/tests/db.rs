use super::*;
use husky_entity_path::{EntityPath, EntityPathPtr, InternEntityPath};
use husky_term::{new_term_itr, AskDecl, TermDb, TermInterner, Ty};
use husky_term::{InternTerm, TermDbStorage};
use husky_word::InternWord;
use salsa::Database;
use std::collections::HashMap;

#[salsa::database(TermDbStorage)]
pub(crate) struct TyInferTestsDb {
    storage: salsa::Storage<Self>,
    term_itr: TermInterner,
    entity_tys: HashMap<EntityPathPtr, Ty>,
}

impl Database for TyInferTestsDb {}

impl InternTerm for TyInferTestsDb {
    fn term_itr(&self) -> &TermInterner {
        todo!()
    }
}

impl InternEntityPath for TyInferTestsDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        todo!()
    }
}

impl InternWord for TyInferTestsDb {
    fn word_allocator(&self) -> &husky_word::WordInterner {
        todo!()
    }
}

impl AskDecl for TyInferTestsDb {
    fn ask_namespace_decl(
        &self,
        namespace: husky_term::TermNamespace,
    ) -> husky_term::TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Ty) -> husky_term::TermResultArc<husky_term::TyDecl> {
        todo!()
    }
}

impl TyInferTestsDb {
    pub(crate) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            term_itr: new_term_itr(),
            entity_tys: Default::default(),
        };
        db.init();
        db
    }

    fn init(&mut self) {
        let term_menu = self.term_menu();
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("i32"))),
                term_menu.i32(),
            )
            .is_none());
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("i64"))),
                term_menu.i64(),
            )
            .is_none());
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("f32"))),
                term_menu.f32(),
            )
            .is_none());
        assert!(self
            .entity_tys
            .insert(
                self.it_entity_path(EntityPath::root(self.it_ident("f64"))),
                term_menu.f64(),
            )
            .is_none());
    }
}

impl TyInferQueries for TyInferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPathPtr) -> Ty {
        self.entity_tys[&entity]
    }
}
