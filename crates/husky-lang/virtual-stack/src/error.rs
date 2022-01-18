#[derive(Debug)]
pub struct VirtualStackError {}

pub type VirtualStackResult<T> = Result<T, VirtualStackError>;
