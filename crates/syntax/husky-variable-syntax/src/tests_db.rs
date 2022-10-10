use husky_word::{InternWord, WordInterner};

use crate::*;

#[salsa::database(VariableQueryStorage)]
pub struct VariableTestsDb {
    storage: salsa::Storage<Self>,
    word_itr: WordInterner,
}

impl salsa::Database for VariableTestsDb {}

impl AnswerVariableQuery for VariableTestsDb {}

impl VariableTestsDb {
    pub(crate) fn new() -> Self {
        Self {
            word_itr: husky_word::new_word_itr(),
            storage: Default::default(),
        }
    }

    fn fake_ctx<'a>(&'a self) -> VariableContext<'a> {
        let mut ctx = VariableContext::new(self);
        /* do something with ctx */
        ctx
    }
}

impl InternWord for VariableTestsDb {
    fn word_allocator(&self) -> &WordInterner {
        &self.word_itr
    }
}
