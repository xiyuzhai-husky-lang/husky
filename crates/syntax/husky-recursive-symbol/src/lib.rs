mod context;
mod sheet;

pub use context::RecursiveSymbolContext;
pub use sheet::RecursiveSymbolSheet;

use husky_entity_path::EntityPath;
use husky_identifier::*;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RecursiveSymbol {
    pub ident: Identifier,
    pub kind: RecursiveSymbolKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RecursiveSymbolKind {
    EntityPath(EntityPath),
    ThisValue,
    ThisMethod,
    ThisField,
}
