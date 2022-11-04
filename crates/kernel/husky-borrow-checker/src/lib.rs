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
    borrows: &'a BorrowTable,
    variables: VariableStack,
    lifetimes: LifetimeStack,
}

impl<'a> BorrowChecker<'a> {
    pub fn new(borrows: &'a BorrowTable) -> Self {
        Self {
            timer: Default::default(),
            borrows,
            variables: Default::default(),
            lifetimes: Default::default(),
        }
    }

    pub(crate) fn new_borrow(
        &mut self,
        variable: VariableIdx,
        borrower: LifetimeIdx,
    ) -> BorrowResult<()> {
        self.variables.new_borrow(variable, &self.timer);
        self.lifetimes.new_borrow(borrower, &self.timer)
    }
}
