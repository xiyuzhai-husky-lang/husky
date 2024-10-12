use crate::*;
use husky_linket_impl::var_id::IsVarId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Windlass<VarId: IsVarId> {
    Specific(VarId),
    Generic {
        page_start: VarId,
        moored: VarId,
        zone: Option<FigureZone>,
        page_limit: Option<usize>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FigureZone {
    Gallery = 1,
    Text,
}
impl FigureZone {
    pub(crate) fn len(self) -> usize {
        match self {
            FigureZone::Gallery => 1,
            FigureZone::Text => 1,
        }
    }
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
            Windlass::Specific(var_id) | Windlass::Generic { moored: var_id, .. } => Some(var_id),
        }
    }
}
