mod action;
mod dependency;
mod resource;
#[cfg(test)]
mod tests;

use action::*;
use dependency::*;
use resource::*;

#[derive(Default)]
pub struct TimeMachine {
    resources: ResourceStack,
    dependencies: DependencyList,
    actions: ActionHistory,
}

impl TimeMachine {
    pub fn new_immutable(&mut self) -> VariableIdx {
        self.resources.new_variable(VariableQualifier::Immutable)
    }
    pub fn new_mutable(&mut self) -> VariableIdx {
        self.resources.new_variable(VariableQualifier::Mutable)
    }

    pub fn new_lifetime(&mut self) -> LifetimeIdx {
        self.resources.new_lifetime()
    }

    pub fn new_borrow(&mut self, variable: VariableIdx, borrower: LifetimeIdx) {
        self.dependencies.new_borrow(variable, borrower);
        self.resources.new_borrow(variable, borrower);
        self.actions.new_borrow(variable, borrower);
    }

    pub fn variable_state(&self, idx: VariableIdx) -> &VariableState {
        self.resources.variable_state(idx)
    }

    pub fn lifetime_state(&self, idx: LifetimeIdx) -> LifetimeState {
        self.resources.lifetime_state(idx)
    }
}
