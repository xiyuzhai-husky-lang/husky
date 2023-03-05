use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermCategory {
    universe: TermUniverse,
}

impl TermCategory {
    pub fn new(universe: TermUniverse) -> Self {
        Self { universe }
    }

    pub fn from_valid(db: &dyn ValidTermDb, valid_term: ValidTermCategory) -> Self {
        TermCategory::new(TermUniverse::from_valid(db, valid_term.universe()))
    }

    pub fn ty(self) -> TermResult<Term> {
        Ok(Self {
            universe: self.universe.next()?,
        }
        .into())
    }

    pub fn universe(&self) -> TermUniverse {
        self.universe
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for TermCategory {
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
//         TermCategory::Type => "Type",
//         TermCategory::Sort => "Sort",
//         TermCategory::Prop => "Prop",
//     }
// }
