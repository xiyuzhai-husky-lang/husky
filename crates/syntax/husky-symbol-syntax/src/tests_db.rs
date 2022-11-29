// use husky_word::{WordDb, dyn WordDb};

// use crate::*;

// pub struct SymbolTestsDb {
//     storage: salsa::Storage<Self>,
//     word_itr: dyn WordDb,
// }

// impl salsa::Database for SymbolTestsDb {}

// impl SymbolQueries for SymbolTestsDb {}

// impl SymbolTestsDb {
//     pub fn new() -> Self {
//         Self {
//             word_itr: dyn WordDb::default(),
//             storage: Default::default(),
//         }
//     }

//     pub fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
//         let ctx = SymbolContext::new(self, &[]);
//         /* do something with ctx */
//         ctx
//     }
// }

// impl WordDb for SymbolTestsDb {
//     fn word_itr(&self) -> &dyn WordDb {
//         &self.word_itr
//     }
// }
