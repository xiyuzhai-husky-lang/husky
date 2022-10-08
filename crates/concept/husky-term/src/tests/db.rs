use husky_entity_path::{new_entity_path_itr, EntityPathInterner, InternEntityPath};

use crate::*;

#[salsa::database(TermQueryStorage)]
pub(crate) struct TermTestsDb {
    storage: salsa::Storage<TermTestsDb>,
    entity_path_itr: EntityPathInterner,
    term_itr: TermInterner,
}

impl TermTestsDb {
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
            entity_path_itr: new_entity_path_itr(),
            term_itr: new_term_itr(),
        }
        .init()
    }

    fn init(mut self) -> Self {
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

impl AskDecl for TermTestsDb {
    fn ask_namespace_decl(&self, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Ty) -> TermResultArc<TyDecl> {
        todo!()
    }
}
