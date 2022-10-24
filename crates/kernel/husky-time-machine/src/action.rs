use crate::*;

#[derive(Default)]
pub struct ActionHistory(Vec<Action>);

pub enum Action {
    NewBorrow {
        variable: VariableIdx,
        borrower: LifetimeIdx,
    },
}

impl ActionHistory {
    pub fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        self.0.push(Action::NewBorrow { variable, borrower })
    }
}
