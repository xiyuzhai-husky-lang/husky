pub mod elaborator;
pub mod error;

use self::error::*;
use super::*;
use crate::{
    expr::VdMirExprIdx,
    region::{VdMirExprRegionData, VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::VdMirStmtIdxRange,
};

pub struct VdMirTacticElaborationTracker {
    history: VdMirTacticElaborationHistory,
    conclusion: Option<VdMirTacticElaborationResult<VdMirTacticElaboration>>,
}

impl VdMirTacticElaborationTracker {
    pub fn new_trivial() -> Self {
        Self {
            history: VdMirTacticElaborationHistory::Trivial,
            conclusion: None,
        }
    }
}

pub enum VdMirTacticElaborationHistory {
    Trivial,
}

pub enum VdMirTacticElaboration {}

impl VdMirTacticElaborationTracker {
    pub fn conclusion(&self) -> Option<VdMirTacticElaborationResultRef<&VdMirTacticElaboration>> {
        self.conclusion.as_ref().map(|result| result.as_ref())
    }
}
