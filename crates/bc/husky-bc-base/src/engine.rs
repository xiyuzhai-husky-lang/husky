use husky_ndp_task::engine::IsNdpEngine;

use crate::{db::BcDb, state::BcState, trajectory::BcTrajectory};

pub struct BcEngine<'a> {
    db: &'a BcDb,
    state: BcState,
    goal: BcState,
    history: BcTrajectory,
}

impl<'a> IsNdpEngine<'a> for BcEngine<'a> {}
