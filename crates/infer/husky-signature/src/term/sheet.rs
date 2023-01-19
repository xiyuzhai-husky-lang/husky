use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermSheet {
    term_symbol_page: TermSymbolPage,
}

impl SignatureTermSheet {
    pub fn new(term_symbol_page: TermSymbolPage) -> Self {
        Self { term_symbol_page }
    }

    pub fn term_symbol_page(&self) -> &TermSymbolPage {
        &self.term_symbol_page
    }
}
