use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct Span {
    start: usize,
    end: usize,
    root: Token,
}
