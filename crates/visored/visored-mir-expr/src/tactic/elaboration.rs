pub mod elaborator;
pub mod error;

use super::*;
use crate::{
    expr::VdMirExprIdx,
    region::{VdMirExprRegionData, VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::VdMirStmtIdxRange,
};

pub enum VdMirTacticElaboration {
    Explicit(VdMirTacticIdxRange),
    Implicit,
    Illicit,
}
