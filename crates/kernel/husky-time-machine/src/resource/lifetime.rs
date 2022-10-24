use super::*;

pub struct LifetimeResource {
    state: LifetimeState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LifetimeState {
    Intact,
    Invalid,
}

impl LifetimeResource {
    pub fn new() -> Self {
        Self {
            state: LifetimeState::Intact,
        }
    }
}

impl ResourceStack {
    pub fn lifetime_state(&self, idx: LifetimeIdx) -> LifetimeState {
        self[idx].state
    }
}
