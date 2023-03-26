use crate::*;

impl RawTerm {
    #[inline(always)]
    pub fn apply(self, db: &dyn RawTermDb, argument: impl Into<RawTerm>) -> Self {
        RawTermExplicitApplication::new(db, self, argument.into()).into()
    }

    pub const PROP: RawTerm = RawTerm::Category(TermCategory::new(TermUniverse::new(0)));

    pub const TYPE: RawTerm = RawTerm::Category(TermCategory::new(TermUniverse::new(1)));
}
