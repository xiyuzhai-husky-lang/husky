#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum VdMirTacticElaborationError {
    #[error("tactic elaboration failed")]
    Failed,
}

pub type VdMirTacticElaborationResult<T> = Result<T, VdMirTacticElaborationError>;
pub type VdMirTacticElaborationResultRef<'a, T> = Result<T, &'a VdMirTacticElaborationError>;
