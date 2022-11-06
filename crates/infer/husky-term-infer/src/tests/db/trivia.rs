use super::*;

impl Upcast<dyn TermDb> for InferTestsDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl Database for InferTestsDb {}

impl InternTerm for InferTestsDb {
    fn term_itr(&self) -> &TermInterner {
        &self.term_itr
    }
}

impl InternEntityPath for InferTestsDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        &self.entity_path_itr
    }
}

impl InternWord for InferTestsDb {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_itr
    }
}

impl SymbolQueries for InferTestsDb {}
