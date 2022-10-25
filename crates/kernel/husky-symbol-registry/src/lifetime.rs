use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LifetimeIdx(usize);

impl SymbolRegistry {
    pub fn new_lifetime(&mut self) -> LifetimeIdx {
        let idx = LifetimeIdx(self.registrations.len());
        self.registrations.push(SymbolRegistration::Lifetime);
        idx
    }
}

impl std::fmt::Display for LifetimeIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "'#".fmt(f)?;
        self.0.fmt(f)
    }
}
