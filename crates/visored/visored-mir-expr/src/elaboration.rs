pub mod error;

use self::error::*;
use super::*;
use crate::{
    expr::VdMirExprIdx,
    region::{VdMirExprRegionData, VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::VdMirStmtIdxRange,
};

#[derive(Debug, PartialEq, Eq)]
pub struct VdMirTracker {
    history: VdMirHistory,
    conclusion: Option<VdMirTacticElaborationResult<VdMirTacticElaboration>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirHistory {
    Trivial,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirTacticElaboration {}

impl VdMirTracker {
    pub fn new_trivial() -> Self {
        Self {
            history: VdMirHistory::Trivial,
            conclusion: None,
        }
    }
}

impl VdMirTracker {
    pub fn conclusion(&self) -> Option<VdMirTacticElaborationResultRef<&VdMirTacticElaboration>> {
        self.conclusion.as_ref().map(|result| result.as_ref())
    }
}
