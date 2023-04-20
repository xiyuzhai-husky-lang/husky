use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar, constructor = new_inner)]
pub struct EtherealTermVariable {
    pub ty: EtherealTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub idx: u8,
}

impl EtherealTermVariable {
    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        variable: DeclarativeTermVariable,
    ) -> TermResult<Self> {
        let ty = variable.ty(db)?;
        let ty =
            EtherealTerm::from_declarative(db, ty, TermTypeExpectation::FinalDestinationEqsSort)?;
        Ok(Self::new_inner(db, ty, variable.idx(db)))
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
