use crate::*;

#[derive(Default)]
pub struct ActionHistory(Vec<Action>);

pub enum Action {
    NewBorrow {
        variable: VariableIdx,
        borrower: LifetimeIdx,
    },
    Init,
}

impl ActionHistory {
    pub fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        self.0.push(Action::NewBorrow { variable, borrower })
    }
}

impl<'a> BorrowChecker<'a> {
    pub(crate) fn act(&mut self, action: Action) -> BorrowResult<()> {
        match action {
            Action::NewBorrow { variable, borrower } => todo!(),
            Action::Init => todo!(),
        }
    }
}
