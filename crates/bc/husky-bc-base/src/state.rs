use husky_ddp_task::state::IsDdpState;

use crate::stmt::BcStmtIdx;

pub struct BcState {
    goal: BcStmtIdx,
    stmts: Vec<BcStmtIdx>,
}

impl IsDdpState for BcState {}
