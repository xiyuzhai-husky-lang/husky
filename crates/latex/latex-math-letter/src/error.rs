use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Error)]
pub enum LxMathLetterError {
    #[error("cannot apply letter style")]
    CannotApplyLetterStyle,
}
