pub mod engine;
pub mod error;
pub mod outcome;
pub mod tactics;

use eterned::db::EternerDb;
use visored_models::VdModels;

#[derive(Clone, Copy)]
pub struct VdTacticsEvaluationBaseEngine<'db> {
    db: &'db EternerDb,
}

impl<'db> VdTacticsEvaluationBaseEngine<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self { db }
    }
}
