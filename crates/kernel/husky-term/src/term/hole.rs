use super::*;

#[salsa::interned(db = TermDb, jar = TermJar, constructor = new_inner)]
pub struct TermHole {
    pub ty: Term,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl TermHole {
    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        raw_term_variable: RawTermHole,
    ) -> TermResult<Self> {
        let ty = raw_term_variable.ty(db)?;
        let ty = Term::from_raw_unchecked(db, ty, TermTypeExpectation::FinalDestinationEqsSort)?;
        Ok(Self::new_inner(db, ty, raw_term_variable.idx(db)))
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        self.ty(db).check(db)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_variable(db, self, f)
    }
}
