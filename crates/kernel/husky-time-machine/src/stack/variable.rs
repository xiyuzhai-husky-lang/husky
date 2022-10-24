use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableResource {
    qual: VariableQualifier,
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
    Outdated,
    Destruct,
    Moved,
}

impl Default for VariableState {
    fn default() -> Self {
        VariableState::Intact
    }
}

impl VariableResource {
    pub fn new(qual: VariableQualifier) -> Self {
        Self {
            qual,
            state: VariableState::default(),
        }
    }
}

impl ResourceStack {
    pub fn variable_state(&self, idx: VariableIdx) -> &VariableState {
        &self[idx].state
    }
}
