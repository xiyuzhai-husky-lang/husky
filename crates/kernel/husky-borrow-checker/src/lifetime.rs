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
    Uninitialized,
    Intact,
    Outdated,
}

impl Default for LifetimeState {
    fn default() -> Self {
        LifetimeState::Uninitialized
    }
}

impl<'a> std::ops::Index<LifetimeIdx> for LifetimeStack {
    type Output = LifetimeEntry;

    fn index(&self, index: LifetimeIdx) -> &Self::Output {
        self.0.iter().find(|entry| entry.idx == index).unwrap()
    }
}
impl<'a> std::ops::IndexMut<LifetimeIdx> for LifetimeStack {
    fn index_mut(&mut self, index: LifetimeIdx) -> &mut Self::Output {
        self.0.iter_mut().find(|entry| entry.idx == index).unwrap()
    }
}
impl<'a> std::ops::Index<LifetimeIdx> for BorrowChecker<'a> {
    type Output = LifetimeEntry;

    fn index(&self, index: LifetimeIdx) -> &Self::Output {
        &self.lifetimes[index]
    }
}

impl LifetimeStack {
    pub(crate) fn new_use(&mut self, lifetime: LifetimeIdx, timer: &Timer) -> BorrowResult<()> {
        timer.update(&mut self[lifetime].db, |state| match state {
            LifetimeState::Uninitialized | LifetimeState::Intact => Ok(LifetimeState::Intact),
            LifetimeState::Outdated => Err(BorrowError::InvalidLifetime),
        })
    }

    pub(crate) fn outdate(&mut self, lifetime: LifetimeIdx, timer: &Timer) {
        timer.set(&mut self[lifetime].db, LifetimeState::Outdated)
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn lifetime_state(&self, idx: LifetimeIdx) -> LifetimeState {
        *self[idx].db.now()
    }

    pub(crate) fn push_lifetime(&mut self, idx: LifetimeIdx) {
        self.lifetimes.0.push({
            let idx = idx;
            LifetimeEntry {
                idx,
                db: self.timer.new_db(),
            }
        })
    }
}
