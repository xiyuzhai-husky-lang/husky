use super::*;
use crate::token::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct Constituent {
    pub span: Span,
    pub children: Vec<Span>,
}
