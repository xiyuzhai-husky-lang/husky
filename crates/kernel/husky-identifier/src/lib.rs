mod db;
mod valid;
// mod decorator;
// mod keyword;
// mod menu;
// mod opr;
// mod pattern;
// mod style;

pub use db::*;
use husky_word::{Word, WordDb};

use std::sync::Arc;
use vec_like::{VecMap, VecPairMap};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Identifier(Word);

impl Identifier {
    fn from_owned(db: &dyn WordDb, data: String) -> Self {
        assert!(crate::valid::is_valid_ident(&data));
        Self(db.it_word_owned(data))
    }

    fn from_borrowed(db: &dyn WordDb, data: &str) -> Self {
        assert!(crate::valid::is_valid_ident(data));
        Self(db.it_word_borrowed(data))
    }

    fn data(self, db: &dyn IdentifierDb) -> &str {
        db.dt_word(self.0)
    }
}

pub type IdentMap<T> = VecMap<Identifier, T>;
pub type IdentArcDict<T> = VecMap<Identifier, Arc<T>>;
pub type IdentPairMap<T> = VecPairMap<Identifier, T>;
