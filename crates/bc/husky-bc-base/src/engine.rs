use husky_ddp_task::engine::IsDdpEngine;

use crate::{db::BcDb, state::BcState, trajectory::BcTrajectory};

pub struct BcEngine<'a> {
    db: &'a BcDb,
    state: BcState,
    goal: BcState,
    history: BcTrajectory,
}

impl<'a> IsDdpEngine<'a> for BcEngine<'a> {}
