use super::*;

impl Upcast<dyn TermDb> for TermPatternInferFakeDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl Database for TermPatternInferFakeDb {}

impl InternTerm for TermPatternInferFakeDb {
    fn term_itr(&self) -> &TermInterner {
        &self.term_itr
    }
}

impl InternEntityPath for TermPatternInferFakeDb {
    fn entity_path_itr(&self) -> &husky_entity_path::EntityPathInterner {
        &self.entity_path_itr
    }
}

impl InternWord for TermPatternInferFakeDb {
    fn word_itr(&self) -> &husky_word::WordInterner {
        &self.word_itr
    }
}

impl SymbolQueries for TermPatternInferFakeDb {}
