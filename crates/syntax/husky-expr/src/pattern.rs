mod decl;
mod region;
mod symbol;

pub use decl::*;
pub use region::*;
pub use symbol::*;

use super::*;
use husky_entity_path::EntityPath;
use husky_token::{AtToken, DotDotToken, IdentToken, TokenStream};
use husky_word::Ident;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;
use parsec::{ParseFromStream, StreamParser};

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

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum PatternExpr {
    /// example: `1`
    Literal(LiteralData),
    /// example: `a`
    Ident {
        modifier: SymbolModifier,
        ident_token: IdentToken,
    },
    /// example: `A::B`
    Entity(EntityPath),
    /// example: `(a, b)`
    Tuple {
        name: Option<EntityPath>,
        fields: PatternExprIdxRange,
    },
    /// example: `C { .. }`
    Struct {
        name: Option<EntityPath>,
        fields: PatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: PatternExprIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident_token: IdentToken,
        asperand_token: AtToken,
        /// example: `1..9`
        src: PatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: PatternExprIdx,
        dot_dot_token: DotDotToken,
        end: PatternExprIdx,
    },
}

pub(crate) type PatternExprArena = Arena<PatternExpr>;
pub type PatternExprIdx = ArenaIdx<PatternExpr>;
pub type PatternExprIdxRange = ArenaIdxRange<PatternExpr>;
pub type PatternExprMap<V> = ArenaMap<PatternExpr, V>;
