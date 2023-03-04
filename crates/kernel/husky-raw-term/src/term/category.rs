use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawTermCategory {
    universe: RawTermUniverse,
}

impl RawTermCategory {
    pub fn new(universe: RawTermUniverse) -> Self {
        Self { universe }
    }

    pub fn ty(self) -> RawTermResult<RawTerm> {
        Ok(Self {
            universe: self.universe.next()?,
        }
        .into())
    }

    pub fn universe(&self) -> RawTermUniverse {
        self.universe
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for RawTermCategory {
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
//         RawTermCategory::Type => "Type",
//         RawTermCategory::Sort => "Sort",
//         RawTermCategory::Prop => "Prop",
//     }
// }
