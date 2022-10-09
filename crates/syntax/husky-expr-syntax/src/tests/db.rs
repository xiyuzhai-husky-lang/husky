use husky_word::{InternWord, WordInterner};

use crate::*;

#[salsa::database(ExprSyntaxQueryStorage)]
pub(crate) struct ExprSyntaxTestsDb {
    storage: salsa::Storage<Self>,
    word_itr: WordInterner,
}

impl salsa::Database for ExprSyntaxTestsDb {}

impl AnswerExprSyntaxQuery for ExprSyntaxTestsDb {}

impl ExprSyntaxTestsDb {
    pub(crate) fn new() -> Self {
        Self {
            word_itr: husky_word::new_word_itr(),
            storage: Default::default(),
        }
    }
}

impl InternWord for ExprSyntaxTestsDb {
    fn word_allocator(&self) -> &WordInterner {
        &self.word_itr
    }
}
