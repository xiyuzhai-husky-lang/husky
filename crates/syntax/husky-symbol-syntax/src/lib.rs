mod context;
mod query;
mod tests_db;

pub use context::*;
pub use query::*;
pub use tests_db::SymbolTestsDb;
pub use tests_db::*;

use husky_entity_path::EntityPathPtr;
use husky_text::TextRange;
use husky_word::CustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawVariable {
    pub ident: CustomIdentifier,
    pub kind: RawVariableKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RawVariableKind {
    EntityPath(EntityPathPtr),
    LocalVariable { init_range: TextRange },
    FrameVariable { init_range: TextRange },
    Unrecognized(CustomIdentifier),
    ThisValue,
    ThisMethod,
    ThisField,
}
