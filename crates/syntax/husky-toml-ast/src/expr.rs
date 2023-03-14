use crate::*;
use ordered_float::OrderedFloat;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlExpr {
    /// Represents a TOML string
    String(Arc<String>),
    /// Represents a TOML integer
    Integer(i64),
    /// Represents a TOML float
    Float(OrderedFloat<f64>),
    /// Represents a TOML boolean
    Boolean(bool),
    /// Represents a TOML datetime
    Datetime(toml_datetime::Datetime),
    /// Represents a TOML array
    Array(TomlExprIdxRange),
    /// Represents a TOML table
    Table(TomlExprIdxRange),
    Err(TomlAstError),
}

pub type TomlExprArena = Arena<TomlExpr>;
pub type TomlExprIdx = ArenaIdx<TomlExpr>;
pub type TomlExprIdxRange = ArenaIdxRange<TomlExpr>;

impl std::ops::FromResidual<TomlAstError> for TomlExpr {
    fn from_residual(error: TomlAstError) -> Self {
        TomlExpr::Err(error)
    }
}
