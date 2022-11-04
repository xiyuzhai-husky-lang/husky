use super::*;

#[derive(Debug, Default)]
pub struct VariableStack(LocalStack<VariableEntry>);

#[derive(Debug)]
pub struct VariableEntry {
    idx: VariableIdx,
    // qual: VariableQualifier,
    db: TimeDb<VariableState>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableQualifier {
    Immutable,
    Mutable,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VariableState {
    Intact,
    Borrowed,
    MutBorrowed,
    Outdated,
    Moved,
}

impl Default for VariableState {
    fn default() -> Self {
        VariableState::Intact
    }
}

impl<'a> std::ops::Index<VariableIdx> for VariableStack {
    type Output = VariableEntry;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        self.0
            .iter()
            .find(|entry| entry.idx == index)
            .expect("variable not found")
    }
}

impl<'a> std::ops::IndexMut<VariableIdx> for VariableStack {
    fn index_mut(&mut self, index: VariableIdx) -> &mut Self::Output {
        self.0
            .iter_mut()
            .find(|entry| entry.idx == index)
            .expect("variable not found")
    }
}

impl<'a> std::ops::Index<VariableIdx> for BorrowChecker<'a> {
    type Output = VariableEntry;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        &self.variables[index]
    }
}

impl VariableStack {
    pub(crate) fn new_borrow(&mut self, variable: VariableIdx, timer: &Timer) {
        let db = &mut self[variable].db;
        timer.set(db, VariableState::Borrowed)
    }

    pub(crate) fn new_borrow_mut(&mut self, variable: VariableIdx, timer: &Timer) {
        let db = &mut self[variable].db;
        let variable_state = db.now();
        timer.set(&mut self[variable].db, VariableState::MutBorrowed)
    }

    pub(crate) fn outdate(&mut self, variable: VariableIdx, timer: &Timer) {
        timer.set(&mut self[variable].db, VariableState::Outdated)
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn variable_state(&self, variable: VariableIdx) -> &VariableState {
        self[variable].db.now()
    }

    pub(crate) fn init_variable(&mut self, variable: VariableIdx) {
        // variable state is always initialized
        self.variables.0.push(VariableEntry {
            idx: variable,
            db: self.timer.new_db(),
        })
    }
}
