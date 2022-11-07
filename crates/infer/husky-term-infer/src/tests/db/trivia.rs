use super::*;

impl Upcast<dyn TermDb> for TermInferTestsDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl Database for TermInferTestsDb {}

impl InternTerm for TermInferTestsDb {
    fn term_itr(&self) -> &TermInterner {
        &self.term_itr
    }
}

impl InternEntityPath for TermInferTestsDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        &self.entity_path_itr
    }
}

impl InternWord for TermInferTestsDb {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_itr
    }
}

impl SymbolQueries for TermInferTestsDb {}
