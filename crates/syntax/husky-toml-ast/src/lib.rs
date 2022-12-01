#![feature(try_trait_v2)]
mod error;
mod line_group;

pub use error::*;

use husky_word::Word;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(PartialEq, Clone, Debug)]
pub enum TomlAst {
    /// Represents a TOML string
    String(String),
    /// Represents a TOML integer
    Integer(i64),
    /// Represents a TOML float
    Float(f64),
    /// Represents a TOML boolean
    Boolean(bool),
    /// Represents a TOML datetime
    Datetime(toml_datetime::Datetime),
    /// Represents a TOML array
    Array(TomlAstIdxRange),
    /// Represents a TOML table
    Table(TomlAstIdxRange),
    Err(TomlAstError),
}

pub type TomlAstArena = Arena<TomlAst>;
pub type TomlAstIdx = ArenaIdx<TomlAst>;
pub type TomlAstIdxRange = ArenaIdxRange<TomlAst>;

impl std::ops::FromResidual<TomlAstError> for TomlAst {
    fn from_residual(error: TomlAstError) -> Self {
        TomlAst::Err(error)
    }
}
