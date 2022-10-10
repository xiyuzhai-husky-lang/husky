#[salsa::query_group(VariableQueryStorage)]
pub trait VariableQuery: AnswerVariableQuery {}

pub trait AnswerVariableQuery {}
