mod arena;
mod db;
mod error;
mod qual;

pub use self::db::*;
pub use self::qual::*;

#[salsa::jar(db = QualDb)]
pub struct QualJar();

use husky_word::*;
use vec_like::VecPairMap;

pub enum QualLiteral {
    Base(BaseQual),
    Parts(PartsQualLiteralIdx),
}

pub struct PartsQualLiteralIdx {}

pub struct PartsQual(VecPairMap<PartIdent, QualLiteral>);
