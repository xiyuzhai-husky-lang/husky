mod context;
mod query;
mod tests_db;

pub use context::*;
pub use query::*;
pub use tests_db::SymbolTestsDb;
pub use tests_db::*;

use husky_entity_path::EntityPathItd;
use husky_text::TextRange;
use husky_word::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    pub ident: Identifier,
    pub kind: SymbolKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    EntityPath(EntityPathItd),
    LocalVariable { init_range: TextRange },
    FrameVariable { init_range: TextRange },
    Unrecognized,
    ThisValue,
    ThisMethod,
    ThisField,
}
