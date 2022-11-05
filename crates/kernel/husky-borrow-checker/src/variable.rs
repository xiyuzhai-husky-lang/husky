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
    ConstBorrowed,
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
    pub(crate) fn set_const_borrowed(&mut self, variable: VariableIdx, timer: &Timer) {
        self.set_state(variable, timer, VariableState::ConstBorrowed)
    }

    pub(crate) fn set_mut_borrowed(&mut self, variable: VariableIdx, timer: &Timer) {
        self.set_state(variable, timer, VariableState::MutBorrowed)
    }

    pub(crate) fn set_outdated(&mut self, variable: VariableIdx, timer: &Timer) {
        self.set_state(variable, timer, VariableState::Outdated)
    }

    pub(crate) fn set_moved(&mut self, variable: VariableIdx, timer: &Timer) {
        self.set_state(variable, timer, VariableState::Moved)
    }

    fn set_state(&mut self, variable: VariableIdx, timer: &Timer, state: VariableState) {
        timer.set(&mut self[variable].db, state)
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
