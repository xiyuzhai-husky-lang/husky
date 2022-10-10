use husky_word::{InternWord, WordInterner};

use crate::*;

#[salsa::database(VariableQueryStorage)]
pub struct SymbolTestsDb {
    storage: salsa::Storage<Self>,
    word_itr: WordInterner,
}

impl salsa::Database for SymbolTestsDb {}

impl AnswerVariableQuery for SymbolTestsDb {}

impl SymbolTestsDb {
    pub(crate) fn new() -> Self {
        Self {
            word_itr: husky_word::new_word_itr(),
            storage: Default::default(),
        }
    }

    fn fake_ctx<'a>(&'a self) -> SymbolContext<'a> {
        let mut ctx = SymbolContext::new(self);
        /* do something with ctx */
        ctx
    }
}

impl InternWord for SymbolTestsDb {
    fn word_allocator(&self) -> &WordInterner {
        &self.word_itr
    }
}
