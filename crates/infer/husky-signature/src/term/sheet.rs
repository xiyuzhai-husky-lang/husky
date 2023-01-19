use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermSheet {
    symbol_term_page: SymbolTermPage,
}

impl SignatureTermSheet {
    pub fn new(symbol_term_page: SymbolTermPage) -> Self {
        Self { symbol_term_page }
    }

    pub fn symbol_term_page(&self) -> &SymbolTermPage {
        &self.symbol_term_page
    }
}
