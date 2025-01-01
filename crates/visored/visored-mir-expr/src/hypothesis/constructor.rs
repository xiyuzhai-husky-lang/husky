use crate::region::VdMirExprRegionDataMut;
use eterned::db::EternerDb;

pub struct VdMirHypothesisConstructor<'db> {
    db: &'db EternerDb,
    region: VdMirExprRegionDataMut<'db>,
}
