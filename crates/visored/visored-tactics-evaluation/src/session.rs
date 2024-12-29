use crate::*;

pub struct VdTacticsEvaluationSession<'db> {
    db: &'db EternerDb,
}

impl<'db> VdTacticsEvaluationSession<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self { db }
    }
}
