// use husky_identifier::{IdentifierDb, dyn IdentifierDb};

// use crate::*;

// pub struct SymbolTestsDb {
//     storage: salsa::Storage<Self>,
//     word_itr: dyn IdentifierDb,
// }

// impl salsa::Database for SymbolTestsDb {}

// impl SymbolQueries for SymbolTestsDb {}

// impl SymbolTestsDb {
//     pub fn new() -> Self {
//         Self {
//             word_itr: dyn IdentifierDb::default(),
//             storage: Default::default(),
//         }
//     }

//     pub fn fake_symbol_ctx<'a>(&'a self) -> SymbolContext<'a> {
//         let ctx = SymbolContext::new(self, &[]);
//         /* do something with ctx */
//         ctx
//     }
// }

// impl IdentifierDb for SymbolTestsDb {
//     fn word_itr(&self) -> &dyn IdentifierDb {
//         &self.word_itr
//     }
// }
