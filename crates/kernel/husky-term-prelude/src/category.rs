use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CategoryTerm {
    universe: UniverseTerm,
}

impl CategoryTerm {
    pub const fn new(universe: UniverseTerm) -> Self {
        Self { universe }
    }

    pub fn from_(db: &::salsa::Db, _term: CategoryTerm) -> Self {
        CategoryTerm::new(UniverseTerm::from_(db, _term.universe()))
    }

    pub fn ty(self) -> TermPreludeResult<CategoryTerm> {
        Ok(Self {
            universe: self.universe.next()?,
        })
    }

    pub fn universe(&self) -> UniverseTerm {
        self.universe
    }

    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for CategoryTerm {
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
