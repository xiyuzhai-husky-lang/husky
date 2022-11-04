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

impl<'a> BorrowChecker<'a> {
    pub fn variable_state(&self, idx: VariableIdx) -> &VariableState {
        self[idx].db.now().unwrap()
    }

    pub fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        let db = &mut self.variables[variable].db;
        let variable_state = db.now().expect("todo");
        match db.now().expect("todo") {
            VariableState::Intact | VariableState::Borrowed => {
                self.timer.set(db, VariableState::Borrowed)
            }
            VariableState::Outdated => todo!(),
            VariableState::Destruct => todo!(),
            VariableState::Moved => todo!(),
        }
    }
    pub(crate) fn init_variable(&mut self, idx: VariableIdx) {
        self.variables.0.push(VariableEntry {
            idx,
            db: self.timer.new_db(),
        })
    }
}
