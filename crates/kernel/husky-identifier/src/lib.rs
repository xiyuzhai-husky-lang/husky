mod db;
// mod decorator;
// mod keyword;
// mod menu;
// mod opr;
// mod pattern;
// mod style;
// mod valid;

pub use db::*;
use husky_word::Word;

use std::sync::Arc;
use vec_like::{VecMap, VecPairMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Identifier(Word);

impl Identifier {
    fn from_owned(db: &dyn IdentifierDb, data: String) -> Self {
        todo!()
    }

    fn from_ref(db: &dyn IdentifierDb, data: &str) -> Self {
        todo!()
    }

    fn data(self, db: &dyn IdentifierDb) -> &str {
        db.dt_word(self.0)
    }
}

pub type IdentMap<T> = VecMap<Identifier, T>;
pub type IdentArcDict<T> = VecMap<Identifier, Arc<T>>;
pub type IdentPairMap<T> = VecPairMap<Identifier, T>;
