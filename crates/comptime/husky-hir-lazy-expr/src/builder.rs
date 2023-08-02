use crate::*;

pub struct HirLazyExprBuilder<'a> {
    db: &'a dyn HirLazyExprDb,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub fn new(db: &'a dyn HirLazyExprDb) -> Self {
        Self { db }
    }

    pub fn finish(self) -> HirLazyExprRegion {
        HirLazyExprRegion::new(self.db)
    }
}
