use crate::*;

pub struct SymbolContext<'a> {
    db: &'a dyn SymbolDb,
}

impl<'a> SymbolContext<'a> {
    pub fn new(db: &'a dyn SymbolDb) -> Self {
        Self { db }
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Symbol {
        // ad hoc
        Symbol {
            ident,
            kind: SymbolKind::Unrecognized,
        }
    }
}
