use crate::{
    region::{VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::VdMirStmtArenaRef,
};
use eterned::db::EternerDb;

pub struct VdMirHypothesisConstructor<'db> {
    db: &'db EternerDb,
    region: VdMirExprRegionDataMut<'db>,
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn new(db: &'db EternerDb, region: VdMirExprRegionDataMut<'db>) -> Self {
        Self { db, region }
    }
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn stmt_arena(&self) -> VdMirStmtArenaRef {
        self.region.stmt_arena.as_arena_ref()
    }

    pub fn region_data(&self) -> VdMirExprRegionDataRef {
        self.region.as_region_data_ref()
    }
}
