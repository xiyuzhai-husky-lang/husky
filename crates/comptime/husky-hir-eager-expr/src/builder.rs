use crate::*;

pub struct HirEagerExprBuilder<'a> {
    db: &'a dyn HirEagerExprDb,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub fn new(db: &'a dyn HirEagerExprDb) -> Self {
        Self { db }
    }

    pub fn finish(self) -> HirEagerExprRegion {
        HirEagerExprRegion::new(self.db)
    }
}
