use crate::{action::BcAction, state::BcState};

pub struct BcHistory {
    initial_state: BcState,
    actions: Vec<BcAction>,
}
