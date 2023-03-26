use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TermCategory {
    universe: TermUniverse,
}

impl TermCategory {
    pub const fn new(universe: TermUniverse) -> Self {
        Self { universe }
    }

    pub fn from_(db: &dyn TermPreludeDb, _term: TermCategory) -> Self {
        TermCategory::new(TermUniverse::from_(db, _term.universe()))
    }

    pub fn ty(self) -> TermPreludeResult<TermCategory> {
        Ok(Self {
            universe: self.universe.next()?,
        })
    }

    pub fn universe(&self) -> TermUniverse {
        self.universe
    }

    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn TermPreludeDb,
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
