pub mod error;

use self::error::*;
use super::*;
use crate::{
    expr::VdMirExprIdx,
    region::{VdMirExprRegionData, VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::VdMirStmtIdxRange,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdMirStmtElaborationTracker {
    history: VdMirStmtElaborationHistory,
    conclusion: Option<VdMirTacticElaborationResult<VdMirTacticElaboration>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirStmtElaborationHistory {
    Trivial,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirTacticElaboration {}

impl VdMirStmtElaborationTracker {
    pub fn new_trivial() -> Self {
        Self {
            history: VdMirStmtElaborationHistory::Trivial,
            conclusion: None,
        }
    }
}

impl VdMirStmtElaborationTracker {
    pub fn conclusion(&self) -> Option<VdMirTacticElaborationResultRef<&VdMirTacticElaboration>> {
        self.conclusion.as_ref().map(|result| result.as_ref())
    }
}
