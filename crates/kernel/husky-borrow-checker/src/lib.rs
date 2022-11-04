mod action;
mod error;
mod instruction;
mod lifetime;
mod table;
#[cfg(test)]
mod tests;
mod time;
mod variable;

pub use error::*;
use husky_print_utils::p;
pub use instruction::*;

use action::*;
use husky_symbol_registry::*;
use lifetime::*;
use local_stack::LocalStack;
use table::*;
use time::*;
use variable::*;

pub struct BorrowChecker<'a> {
    timer: Timer,
    dependencies: &'a DependencyTable,
    variables: VariableStack,
    lifetimes: LifetimeStack,
}

impl<'a> BorrowChecker<'a> {
    pub fn new(borrows: &'a DependencyTable) -> Self {
        Self {
            timer: Default::default(),
            dependencies: borrows,
            variables: Default::default(),
            lifetimes: Default::default(),
        }
    }

    pub(crate) fn sim_borrow(
        &mut self,
        variable: VariableIdx,
        borrower: LifetimeIdx,
    ) -> BorrowResult<()> {
        self.variables.new_borrow(variable, &self.timer);
        self.lifetimes.new_use(borrower, &self.timer)
    }

    pub(crate) fn sim_borrow_mut(
        &mut self,
        variable: VariableIdx,
        lifetime: LifetimeIdx,
    ) -> BorrowResult<()> {
        match self.variable_state(variable) {
            VariableState::Intact => todo!(),
            VariableState::Borrowed => self.outdate_dependants(variable.into()),
            VariableState::MutBorrowed => todo!(),
            VariableState::Outdated => todo!(),
            VariableState::Destruct => todo!(),
            VariableState::Moved => todo!(),
        }
        self.variables.new_borrow_mut(variable, &self.timer);
        self.lifetimes.new_use(lifetime, &self.timer)
    }

    fn outdate_dependants(&mut self, dependee: BorrowObject) {
        for dependant in self.dependencies.dependants(dependee) {
            self.outdate(dependant)
        }
    }

    fn outdate(&mut self, object: BorrowObject) {
        match object {
            BorrowObject::Variable(variable) => match self.variable_state(variable) {
                VariableState::Intact | VariableState::Borrowed => {
                    self.variables.outdate(variable, &self.timer)
                }
                VariableState::MutBorrowed => todo!(),
                VariableState::Outdated | VariableState::Moved => return,
                VariableState::Destruct => todo!(),
            },
            BorrowObject::Lifetime(lifetime) => self.lifetimes.outdate(lifetime, &self.timer),
        }
        self.outdate_dependants(object)
    }
}
