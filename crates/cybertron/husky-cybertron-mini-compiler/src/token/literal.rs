#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Literal {
    Int(i32),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(i) => i.fmt(f),
        }
    }
}
