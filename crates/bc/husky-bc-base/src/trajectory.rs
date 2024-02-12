use crate::{action::BcAction, state::BcState};
use husky_ndp_task::trajectory::IsTrajectory;

pub struct BcTrajectory {
    initial_state: BcState,
    actions: Vec<BcAction>,
}

impl IsTrajectory for BcTrajectory {}
