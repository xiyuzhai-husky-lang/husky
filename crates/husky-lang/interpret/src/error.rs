#[derive(Debug)]
pub struct InterpretStackError {}

pub type InterpretResult<T> = Result<T, InterpretStackError>;
