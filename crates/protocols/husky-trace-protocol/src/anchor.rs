use husky_linket_impl::var::IsVarId;
use serde::{Deserialize, Serialize};

use crate::windlass::Windlass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Anchor<VarId: IsVarId> {
    Specific(VarId),
    Generic { limit: usize },
}

impl<VarId: IsVarId> From<Windlass<VarId>> for Anchor<VarId> {
    fn from(windlass: Windlass<VarId>) -> Self {
        match windlass {
            Windlass::Specific(var_id) => Anchor::Specific(var_id),
            Windlass::Generic { base, limit } => Anchor::Generic { limit },
        }
    }
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
