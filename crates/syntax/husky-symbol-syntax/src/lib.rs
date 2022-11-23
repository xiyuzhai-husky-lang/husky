mod context;
mod sheet;

pub use context::SymbolContext;
pub use sheet::SymbolSheet;

use husky_entity_path::EntityPath;
use husky_identifier::*;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Symbol {
    pub ident: Identifier,
    pub kind: SymbolKind,
    pub init_range: TextRange,
    pub access_range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    EntityPath(EntityPath),
    LocalVariable { init_range: TextRange },
    FrameVariable { init_range: TextRange },
    Unrecognized,
    ThisValue,
    ThisMethod,
    ThisField,
}
