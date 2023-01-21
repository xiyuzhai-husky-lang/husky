mod decl;
mod expr;
mod page;
mod symbol;

pub use decl::*;
pub use expr::*;
pub use page::*;
pub use symbol::*;

use super::*;
use husky_entity_path::EntityPath;
use husky_token::{AtToken, DotDotToken, IdentifierToken, TokenStream};
use husky_word::Identifier;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;
use parsec::{ParseContext, ParseFrom};

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralData {
    NotNanFloat,
    NotNanF32(NotNan<f32>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PatternExprInfo {
    Parameter,
    Let,
    Match,
    Be,
}
