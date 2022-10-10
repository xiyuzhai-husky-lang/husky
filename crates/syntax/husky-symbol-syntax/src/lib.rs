mod context;
mod query;
#[cfg(test)]
mod tests_db;

pub use context::*;
pub use query::*;
#[cfg(test)]
pub use tests_db::SymbolTestsDb;
#[cfg(test)]
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
