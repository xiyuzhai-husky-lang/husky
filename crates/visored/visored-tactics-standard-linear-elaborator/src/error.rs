#[derive(Debug, thiserror::Error)]
pub enum VdTacticsEvaluationError {}

pub type VdTacticsEvaluationResult<T> = Result<T, VdTacticsEvaluationError>;
