mod action;
mod dependence;
mod stack;
#[cfg(test)]
mod tests;

use action::*;
use dependence::*;
use stack::*;

#[derive(Default)]
pub struct TimeMachine {
    stack: ResourceStack,
    rules: DependencyList,
}

impl TimeMachine {
    pub fn new_immutable(&mut self) -> VariableIdx {
        self.stack.new_variable(VariableQualifier::Immutable)
    }
    pub fn new_mutable(&mut self) -> VariableIdx {
        self.stack.new_variable(VariableQualifier::Mutable)
    }

    pub fn new_lifetime(&mut self) -> LifetimeIdx {
        self.stack.new_lifetime()
    }

    pub fn variable_state(&self, idx: VariableIdx) -> &VariableState {
        self.stack.variable_state(idx)
    }

    pub fn lifetime_state(&self, idx: LifetimeIdx) -> LifetimeState {
        self.stack.lifetime_state(idx)
    }
}
