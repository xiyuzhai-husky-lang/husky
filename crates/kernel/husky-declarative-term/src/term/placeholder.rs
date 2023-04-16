mod set;
mod turn;

pub use self::set::*;
pub use self::turn::*;

use super::*;
use thiserror::Error;

/// variables are derived from symbols
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new)]
pub struct DeclarativeTermPlaceholder {
    pub ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl DeclarativeTermPlaceholder {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("v{}", self.idx(db)))
    }
}
