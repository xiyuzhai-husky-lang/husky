mod contract;
mod region;
mod symbol;

pub use self::region::*;
pub use self::symbol::*;

use super::*;
use husky_coword::Ident;
use husky_entity_path::ItemPath;
use husky_token::{AtToken, DotDotToken, IdentToken, TokenStream};
use idx_arena::{ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;
use parsec::{StreamParser, TryParseOptionFromStream};

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralData {
    NotNanFloat,
    NotNanF32(NotNan<f32>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PatternSynExprInfo {
    Parameter,
    Let,
    Match,
    Be,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub enum PatternSynExpr {
    /// example: `1`
    Literal(LiteralData),
    /// example: `a`
    Ident {
        symbol_modifier_keyword_group: Option<SymbolModifierKeywordGroup>,
        ident_token: IdentToken,
    },
    /// example: `A::B`
    Entity(ItemPath),
    /// example: `(a, b)`
    Tuple {
        name: Option<ItemPath>,
        fields: PatternSynExprIdxRange,
    },
    /// example: `C { .. }`
    Struct {
        name: Option<ItemPath>,
        fields: PatternSynExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: PatternSynExprIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident_token: IdentToken,
        asperand_token: AtToken,
        /// example: `1..9`
        src: PatternSynExprIdx,
    },
    /// example: `1..9`
    Range {
        start: PatternSynExprIdx,
        dot_dot_token: DotDotToken,
        end: PatternSynExprIdx,
    },
}

pub(crate) type PatternSynExprArena = Arena<PatternSynExpr>;
pub type PatternSynExprIdx = ArenaIdx<PatternSynExpr>;
pub type PatternSynExprIdxRange = ArenaIdxRange<PatternSynExpr>;
pub type PatternSynExprMap<V> = ArenaMap<PatternSynExpr, V>;
pub type PatternSynExprOrderedMap<V> = ArenaOrderedMap<PatternSynExpr, V>;
