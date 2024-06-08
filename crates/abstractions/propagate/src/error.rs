#[derive(Debug, PartialEq, Eq)]
pub enum PropagationError {
    InfiniteLoop,
}

pub type PropagationResult<T> = Result<T, PropagationError>;
pub type PropagationResultRef<'a, T> = Result<T, &'a PropagationError>;
