use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreciseTermCategory {
    universe: PreciseTermUniverse,
}

impl PreciseTermCategory {
    #[inline(always)]
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTermCategory,
        raw_ty_expectation: TermTypeExpectation,
    ) -> Self {
        PreciseTermCategory {
            universe: PreciseTermUniverse::from_raw(db, raw_term.universe(), raw_ty_expectation),
        }
    }

    pub fn new(universe: PreciseTermUniverse) -> Self {
        Self { universe }
    }

    pub fn ty(self) -> PreciseTermResult<PreciseTerm> {
        Ok(Self {
            universe: self.universe.next()?,
        }
        .into())
    }

    pub fn universe(&self) -> PreciseTermUniverse {
        self.universe
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for PreciseTermCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.universe.raw() {
            0 => f.write_str("Prop"),
            1 => f.write_str("Type"),
            u => {
                f.write_str("Type ");
                std::fmt::Display::fmt(&(u - 1), f)
            }
        }
    }
}
// pub fn as_str(self) -> &'static str {
//     match self {
//         PreciseTermCategory::Type => "Type",
//         PreciseTermCategory::Sort => "Sort",
//         PreciseTermCategory::Prop => "Prop",
//     }
// }
