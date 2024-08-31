use crate::*;
use husky_linket_impl::var_id::IsVarId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Windlass<VarId: IsVarId> {
    Specific(VarId),
    Generic {
        base: Option<VarId>,
        limit: Option<usize>,
    },
}

impl<VarId: IsVarId> Windlass<VarId> {
    pub fn is_specific(self) -> bool {
        match self {
            Windlass::Specific(_) => true,
            _ => false,
        }
    }

    pub fn is_generic(self) -> bool {
        match self {
            Windlass::Generic { .. } => true,
            _ => false,
        }
    }

    pub fn var_id(self) -> Option<VarId> {
        match self {
            Windlass::Specific(var_id)
            | Windlass::Generic {
                base: Some(var_id), ..
            } => Some(var_id),
            Windlass::Generic { base: None, .. } => None,
        }
    }
}
