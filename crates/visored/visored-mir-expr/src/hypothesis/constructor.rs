use crate::region::VdMirExprRegionDataMut;
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
