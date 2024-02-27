use super::*;

/// `Sort u` for some universe `u`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sort {
    universe: Universe,
}

impl Sort {
    pub const PROP: Self = Sort {
        universe: Universe::PROP_UNIVERSE,
    };
    pub const TYPE: Self = Sort {
        universe: Universe::TYPE_UNIVERSE,
    };

    pub const fn new(universe: Universe) -> Self {
        Self { universe }
    }

    pub fn from_(db: &::salsa::Db, _term: Sort) -> Self {
        Sort::new(Universe::from_(db, _term.universe()))
    }

    pub fn ty(self) -> TermPreludeResult<Sort> {
        Ok(Self {
            universe: self.universe.next()?,
        })
    }

    pub fn universe(&self) -> Universe {
        self.universe
    }

    pub fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.universe.raw() {
            0 => f.write_str("Prop"),
            1 => f.write_str("Type"),
            u => {
                f.write_str("Type ")?;
                std::fmt::Display::fmt(&(u - 1), f)
            }
        }
    }
}
