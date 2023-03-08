mod db;
mod error;
mod qual;

pub use self::db::*;
pub use self::qual::*;

#[salsa::jar(db = QualDb)]
pub struct QualJar(Qual);

use husky_word::*;
use vec_like::VecPairMap;
