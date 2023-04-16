use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermPlaceholder {
    pub ty: EtherealTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl EtherealTermPlaceholder {
    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        raw_term_variable: RawTermPlaceholder,
    ) -> TermResult<Self> {
        let ty = raw_term_variable.ty(db)?;
        let ty =
            EtherealTerm::from_raw_unchecked(db, ty, TermTypeExpectation::FinalDestinationEqsSort)?;
        Ok(Self::new_inner(db, ty, raw_term_variable.idx(db)))
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_variable(db, self, f)
    }
}
