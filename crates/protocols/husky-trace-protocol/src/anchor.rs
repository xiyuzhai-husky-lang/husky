use husky_linket_impl::var::IsVarId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Anchor<VarId: IsVarId> {
    Specific(VarId),
    Generic { limit: usize },
}

impl<VarId: IsVarId> Anchor<VarId> {
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
