use husky_word::{InternWord, WordInterner};

use crate::*;

#[salsa::database(SymbolDbStorage)]
pub struct SymbolTestsDb {
    storage: salsa::Storage<Self>,
    word_itr: WordInterner,
}

impl salsa::Database for SymbolTestsDb {}

impl SymbolQueries for SymbolTestsDb {}

impl SymbolTestsDb {
    pub fn new() -> Self {
        Self {
            word_itr: WordInterner::default(),
            storage: Default::default(),
        }
    }

    pub fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
        let ctx = SymbolContext::new(self, &[]);
        /* do something with ctx */
        ctx
    }
}

impl InternWord for SymbolTestsDb {
    fn word_itr(&self) -> &WordInterner {
        &self.word_itr
    }
}
