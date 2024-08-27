#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor<StaticVarId> {
    Specific(StaticVarId),
    Generic { limit: usize },
}

impl<StaticVarId> Anchor<StaticVarId> {
    pub fn is_generic(self) -> bool {
        matches!(self, Anchor::Generic { .. })
    }

    pub fn generic_limit(self) -> Option<usize> {
        match self {
            Anchor::Specific(_) => None,
            Anchor::Generic { limit } => Some(limit),
        }
    }
}
