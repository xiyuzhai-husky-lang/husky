#[derive(Debug, thiserror::Error)]
pub enum VdMirTacticEvaluationError {}

pub type VdMirTacticEvaluationResult<T> = Result<T, VdMirTacticEvaluationError>;
