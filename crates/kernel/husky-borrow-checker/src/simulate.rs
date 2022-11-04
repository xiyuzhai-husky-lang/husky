use crate::*;

impl<'a> BorrowChecker<'a> {
    pub(crate) fn sim_borrow(
        &mut self,
        variable: VariableIdx,
        borrower: LifetimeIdx,
    ) -> BorrowResult<()> {
        match self.variable_state(variable) {
            VariableState::Intact | VariableState::Borrowed => (),
            VariableState::MutBorrowed => self.outdate_dependants(variable.into()),
            VariableState::Outdated => Err(BorrowError::BorrowOutdatedVariable)?,
            VariableState::Moved => Err(BorrowError::BorrowMovedVariable)?,
        }
        self.variables.new_borrow(variable, &self.timer);
        self.lifetimes.new_borrow(borrower, &self.timer)
    }

    pub(crate) fn sim_borrow_mut(
        &mut self,
        variable: VariableIdx,
        lifetime: LifetimeIdx,
    ) -> BorrowResult<()> {
        match self.variable_state(variable) {
            VariableState::Intact => (),
            VariableState::Borrowed | VariableState::MutBorrowed => {
                self.outdate_dependants(variable.into())
            }
            VariableState::Outdated => Err(BorrowError::BorrowOutdatedVariable)?,
            VariableState::Moved => Err(BorrowError::BorrowMovedVariable)?,
        }
        self.variables.new_borrow_mut(variable, &self.timer);
        self.lifetimes.new_borrow(lifetime, &self.timer)
    }

    pub(crate) fn sim_move(&mut self, variable: VariableIdx) {
        self.outdate_dependants(variable.into());
        self.variables.set_moved(variable, &self.timer)
    }

    fn outdate_dependants(&mut self, dependee: BorrowObject) {
        for dependant in self.dependencies.dependants(dependee) {
            self.outdate(dependant)
        }
    }

    fn outdate(&mut self, object: BorrowObject) {
        match object {
            BorrowObject::Variable(variable) => match self.variable_state(variable) {
                VariableState::Intact | VariableState::Borrowed | VariableState::MutBorrowed => {
                    self.variables.set_outdated(variable, &self.timer)
                }
                VariableState::Outdated | VariableState::Moved => return,
            },
            BorrowObject::Lifetime(lifetime) => match self.lifetime_state(lifetime) {
                LifetimeState::Uninitialized | LifetimeState::Outdated => return,
                LifetimeState::Intact => self.lifetimes.set_outdated(lifetime, &self.timer),
            },
        }
        self.outdate_dependants(object)
    }
}
