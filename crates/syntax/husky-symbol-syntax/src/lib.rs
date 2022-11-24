mod context;
mod db;
mod jar;
mod sheet;

pub use context::SymbolContext;
pub use db::SymbolDb;
pub use jar::SymbolJar;
pub use sheet::SymbolSheet;

use husky_entity_path::EntityPath;
use husky_identifier::*;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Symbol {
    pub ident: Identifier,
    pub kind: SymbolKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    EntityPath(EntityPath),
    LocalVariable { init_range: TextRange },
    FrameVariable { init_range: TextRange },
    ThisValue,
    ThisMethod,
    ThisField,
}
