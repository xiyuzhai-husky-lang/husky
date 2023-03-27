use super::*;
use thiserror::Error;

/// variables are derived from symbols
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermVariable {
    pub ty: RawTermSymbolTypeResult<RawTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl RawTermVariable {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
        // ctx.fmt_abstract_symbol(db, self, f)
    }
}
