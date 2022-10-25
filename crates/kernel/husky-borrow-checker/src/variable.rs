use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableEntry {
    idx: VariableIdx,
    // qual: VariableQualifier,
    state: VariableState,
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

// impl VariableEntry {
//     pub fn new(idx: VariableIdx, qual: VariableQualifier) -> Self {
//         Self {
//             idx,
//             // qual,
//             state: VariableState::default(),
//         }
//     }
// }

impl<'a> std::ops::Index<VariableIdx> for BorrowChecker<'a> {
    type Output = VariableEntry;

    fn index(&self, index: VariableIdx) -> &Self::Output {
        self.variables
            .iter()
            .find(|entry| entry.idx == index)
            .unwrap()
    }
}

impl<'a> std::ops::IndexMut<VariableIdx> for BorrowChecker<'a> {
    fn index_mut(&mut self, index: VariableIdx) -> &mut Self::Output {
        self.variables
            .iter_mut()
            .find(|entry| entry.idx == index)
            .unwrap()
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn variable_state(&self, idx: VariableIdx) -> &VariableState {
        &self[idx].state
    }

    pub fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        let variable_state = &mut self[variable].state;
        match variable_state {
            VariableState::Intact | VariableState::Borrowed => {
                *variable_state = VariableState::Borrowed
            }
            VariableState::Outdated => todo!(),
            VariableState::Destruct => todo!(),
            VariableState::Moved => todo!(),
        }
    }
    pub (crate) fn init_variable(&mut self,idx: VariableIdx) {
        self.variables.push(VariableEntry { idx , state: Default::default() })
    }
}

