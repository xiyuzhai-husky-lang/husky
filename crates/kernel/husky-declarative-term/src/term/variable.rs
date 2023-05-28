mod set;
mod turn;

pub use self::set::*;
pub use self::turn::*;

use super::*;
use thiserror::Error;

/// variables are externalized symbols, derived from symbols, and defined in a bottom-up manner
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new)]
pub struct DeclarativeTermVariable {
    pub ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    /// this is the index to disambiguate it from all other symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl DeclarativeTermVariable {
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
