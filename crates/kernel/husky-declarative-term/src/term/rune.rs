mod r#abstract;
mod set;

pub use self::r#abstract::*;
pub use self::set::*;

use super::*;


/// variables are externalized symbols, derived from symbols, and defined in a bottom-up manner
///
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermRune {
    pub ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    /// this is the index to disambiguate it from all other symbols with the same type
    /// so that we have better cache hits
    /// todo: change to RefinedDeBrujinIndex
    pub idx: u8,
}

pub enum RefinedDeBrujinIndex {}

impl DeclarativeTermRune {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("v{}", self.idx(db)))
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermRune {
    fn substitute(
        self,
        _db: &dyn DeclarativeTermDb,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}
