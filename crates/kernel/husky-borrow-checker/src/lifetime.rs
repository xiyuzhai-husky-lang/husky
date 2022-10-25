use super::*;

pub struct LifetimeEntry {
    idx: LifetimeIdx,
    log: LifetimeLog,
}

pub type LifetimeLog = Vec<(usize, LifetimeState)>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LifetimeState {
    Intact,
    Invalid,
}

impl<'a> std::ops::Index<LifetimeIdx> for BorrowChecker<'a> {
    type Output = LifetimeEntry;

    fn index(&self, index: LifetimeIdx) -> &Self::Output {
        self.lifetimes
            .iter()
            .find(|entry| entry.idx == index)
            .unwrap()
    }
}

impl LifetimeEntry {
    pub fn new(idx: LifetimeIdx) -> Self {
        Self { idx, log: vec![] }
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn lifetime_state(&self, idx: LifetimeIdx) -> Option<LifetimeState> {
        self[idx].log.last().map(|(_, state)| *state)
    }

    pub (crate) fn init_lifetime(&mut self,idx: LifetimeIdx) {
        self.lifetimes.push(LifetimeEntry { idx, log: Default::default()})
    }
}
