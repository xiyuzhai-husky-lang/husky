use std::{collections::HashMap, sync::Arc};

use husky_entity_path::{new_entity_path_itr, EntityPathInterner, InternEntityPath};
use husky_word::{InternWord, WordInterner};

use crate::*;

#[salsa::database(TermDbStorage)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
    entity_path_itr: EntityPathInterner,
    term_itr: TermInterner,
    word_itr: WordInterner,
    ty_decls: HashMap<Ty, Arc<TyDecl>>,
}

impl TermTestsDb {
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
            entity_path_itr: new_entity_path_itr(),
            term_itr: Default::default(),
            word_itr: Default::default(),
            ty_decls: Default::default(),
        }
        .init()
    }

    fn init(mut self) -> Self {
        use TyFamily::*;
        let menu = self.term_menu();
        self.ty_decls.extend(menu.primitive_ty_decls());
        self
    }
}

impl salsa::Database for TermTestsDb {}

impl InternEntityPath for TermTestsDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        &self.entity_path_itr
    }
}

impl InternTerm for TermTestsDb {
    fn term_itr(&self) -> &TermInterner {
        &self.term_itr
    }
}

impl InternWord for TermTestsDb {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_itr
    }
}

impl AskDecl for TermTestsDb {
    fn ask_namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl> {
        Ok(self.ty_decls[&ty].clone())
    }

    fn ask_decl(&self, entity_path: EntityPathItd) -> TermResultArc<Decl> {
        todo!()
    }
}
