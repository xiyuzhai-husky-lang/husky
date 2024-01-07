mod r#abstract;
mod set;

use crate::helpers::DeclarativeTermFamily;

pub use self::set::*;

use super::*;

/// variables are externalized symbols, derived from symbols, and defined in a bottom-up manner
///
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new_inner)]
pub struct DeclarativeTermRune {
    pub ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    /// this is the index to disambiguate it from all other symbols with the same type
    /// so that we have better cache hits
    /// todo: change to RefinedDeBrujinIndex
    pub idx: RuneIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuneIndex {
    ty_family: DeclarativeTermFamily,
    disambiguator: u8,
}

impl RuneIndex {
    pub fn ty_family(self) -> DeclarativeTermFamily {
        self.ty_family
    }

    pub fn disambiguator(self) -> u8 {
        self.disambiguator
    }
}

impl std::fmt::Display for RuneIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.disambiguator, f)
    }
}

impl DeclarativeTermRune {
    pub fn new(
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
        disambiguator: u8,
        db: &::salsa::Db,
    ) -> Self {
        Self::new_inner(
            db,
            ty,
            RuneIndex {
                ty_family: match ty {
                    Ok(ty) => ty.family(db),
                    Err(_) => todo!(),
                },
                disambiguator,
            },
        )
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("v{}", self.idx(db).disambiguator))
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermRune {
    fn substitute_copy(
        self,
        _db: &::salsa::Db,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}
