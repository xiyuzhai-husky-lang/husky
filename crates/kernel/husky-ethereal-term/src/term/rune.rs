use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermRune {
    pub ty: EtherealTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

pub enum RuneIndex {}

impl EtherealTermRune {
    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        variable: DeclarativeTermRune,
    ) -> EtherealTermResult<Self> {
        let ty = variable.ty(db)?;
        let ty = EtherealTerm::ty_from_declarative(db, ty)?;
        Ok(Self::new_inner(db, ty, variable.idx(db)))
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_variable(db, self, f)
    }
}
