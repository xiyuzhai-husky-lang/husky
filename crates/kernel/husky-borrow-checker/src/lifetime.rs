use super::*;

#[derive(Debug, Default)]
pub struct LifetimeStack(LocalStack<LifetimeEntry>);

#[derive(Debug)]
pub struct LifetimeEntry {
    idx: LifetimeIdx,
    db: TimeDb<LifetimeState>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LifetimeState {
    Intact,
    Invalid,
}

impl<'a> std::ops::Index<LifetimeIdx> for LifetimeStack {
    type Output = LifetimeEntry;

    fn index(&self, index: LifetimeIdx) -> &Self::Output {
        self.0.iter().find(|entry| entry.idx == index).unwrap()
    }
}

impl<'a> std::ops::Index<LifetimeIdx> for BorrowChecker<'a> {
    type Output = LifetimeEntry;

    fn index(&self, index: LifetimeIdx) -> &Self::Output {
        &self.lifetimes[index]
    }
}

impl LifetimeEntry {
    pub fn new(idx: LifetimeIdx) -> Self {
        Self {
            idx,
            db: TimeDb::new_uninitialized(),
        }
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn lifetime_state(&self, idx: LifetimeIdx) -> Option<LifetimeState> {
        self[idx].db.now().copied()
    }

    pub(crate) fn init_lifetime(&mut self, idx: LifetimeIdx) {
        self.lifetimes.0.push(LifetimeEntry::new(idx))
    }
}
