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
    Outdated,
    Destruct,
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
        let variable_state = db.now();
        match db.now() {
            VariableState::Intact | VariableState::Borrowed => {
                timer.set(db, VariableState::Borrowed)
            }
            VariableState::Outdated => todo!(),
            VariableState::Destruct => todo!(),
            VariableState::Moved => todo!(),
        }
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn variable_state(&self, idx: VariableIdx) -> &VariableState {
        self[idx].db.now()
    }

    pub(crate) fn init_variable(&mut self, idx: VariableIdx) {
        // variable state is always initialized
        self.variables.0.push(VariableEntry {
            idx,
            db: self.timer.new_db(),
        })
    }
}
