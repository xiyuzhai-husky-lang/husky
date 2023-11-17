pub enum IOError {}

pub type IOResult<T> = Result<T, IOError>;
