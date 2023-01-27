#[derive(Debug)]
pub enum PropagationError {
    InfiniteLoop,
}

pub type PropagationResult<T> = Result<T, PropagationError>;
