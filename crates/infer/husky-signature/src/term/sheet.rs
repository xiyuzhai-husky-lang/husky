use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermSheet {
    symbol_term_registry: SymbolTermRegistry,
}

impl SignatureTermSheet {
    pub fn new(symbol_term_registry: SymbolTermRegistry) -> Self {
        Self {
            symbol_term_registry,
        }
    }

    pub fn symbol_term_registry(&self) -> &SymbolTermRegistry {
        &self.symbol_term_registry
    }
}
