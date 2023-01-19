mod error;
mod instruction;
mod lifetime;
mod simulate;
mod table;
#[cfg(test)]
mod tests;
mod time;
mod variable;

pub use error::*;

pub use instruction::*;

use husky_symbol_page::*;
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
}
