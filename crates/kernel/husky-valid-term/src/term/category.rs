use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidTermCategory {
    universe: ValidTermUniverse,
}

impl ValidTermCategory {
    pub fn new(universe: ValidTermUniverse) -> Self {
        Self { universe }
    }

    pub fn ty(self) -> ValidTermResult<ValidTerm> {
        Ok(Self {
            universe: self.universe.next()?,
        }
        .into())
    }

    pub fn universe(&self) -> ValidTermUniverse {
        self.universe
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for ValidTermCategory {
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
//         ValidTermCategory::Type => "Type",
//         ValidTermCategory::Sort => "Sort",
//         ValidTermCategory::Prop => "Prop",
//     }
// }
