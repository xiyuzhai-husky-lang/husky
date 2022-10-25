use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct VariableIdx(usize);

impl SymbolRegistry {
    pub fn new_variable(&mut self) -> VariableIdx {
        let idx = VariableIdx(self.registrations.len());
        self.registrations.push(SymbolRegistration::Variable);
        idx
    }
}

impl std::fmt::Display for VariableIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "#".fmt(f)?;
        self.0.fmt(f)
    }
}
