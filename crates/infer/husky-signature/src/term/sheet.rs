use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermSheet {
    term_symbol_region: TermSymbolRegion,
}

impl SignatureTermSheet {
    pub fn new(term_symbol_region: TermSymbolRegion) -> Self {
        Self { term_symbol_region }
    }

    pub fn term_symbol_region(&self) -> &TermSymbolRegion {
        &self.term_symbol_region
    }
}
