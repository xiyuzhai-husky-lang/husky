#[derive(Debug, PartialEq, Eq)]
pub enum CorgiConfigAstError {}

pub type CorgiConfigAstResult<T> = Result<T, CorgiConfigAstError>;

pub type CorgiConfigAstResultRef<'a, T> = Result<T, &'a CorgiConfigAstError>;
