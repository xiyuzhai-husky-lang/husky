#[salsa::query_group(ExprSyntaxQueryStorage)]
pub trait ExprSyntaxQuery: AnswerExprSyntaxQuery {}

pub trait AnswerExprSyntaxQuery {}
