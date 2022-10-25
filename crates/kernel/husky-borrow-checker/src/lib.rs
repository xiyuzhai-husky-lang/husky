mod action;
mod error;
mod instruction;
mod lifetime;
mod table;
#[cfg(test)]
mod tests;
mod variable;

pub use error::*;
pub use instruction::*;

use action::*;
use husky_symbol_registry::*;
use lifetime::*;
use local_stack::LocalStack;
use table::*;
use variable::*;

pub struct BorrowChecker<'a> {
    borrows: &'a BorrowTable,
    variables: LocalStack<VariableEntry>,
    lifetimes: LocalStack<LifetimeEntry>,
}

impl<'a> BorrowChecker<'a> {
    pub fn new(borrows: &'a BorrowTable) -> Self {
        Self {
            borrows,
            variables: Default::default(),
            lifetimes: Default::default(),
        }
    }
}
