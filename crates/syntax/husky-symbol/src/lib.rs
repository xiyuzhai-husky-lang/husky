mod binding;
mod context;
mod label;
mod lifetime;
mod sheet;
mod variable;

pub use binding::*;
pub use context::SymbolContext;
pub use label::*;
pub use lifetime::*;
pub use sheet::{LocalSymbolSheet, VariableIdx};
pub use variable::*;

use husky_entity_path::EntityPath;
use husky_entity_tree::EntitySymbol;
use husky_text::TextRange;
use husky_vfs::ModulePath;
use husky_word::*;
use idx_arena::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Entity(EntityPath),
    Variable(VariableIdx),
    Lifetime(LifetimeIdx),
    Label(LabelIdx),
}
