mod db;
// mod decorator;
// mod keyword;
// mod menu;
// mod opr;
// mod pattern;
// mod style;
// mod valid;

pub use db::*;

use std::sync::Arc;
use vec_like::{VecMap, VecPairMap};

#[salsa::interned(jar = IdentifierJar)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Identifier {
    #[return_ref]
    data: String,
}

pub type IdentMap<T> = VecMap<Identifier, T>;
pub type IdentArcDict<T> = VecMap<Identifier, Arc<T>>;
pub type IdentPairMap<T> = VecPairMap<Identifier, T>;

#[salsa::jar(db = IdentifierDb)]
pub struct IdentifierJar(Identifier);
