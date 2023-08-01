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
pub enum SynPatternExprInfo {
    Parameter,
    Let,
    Match,
    Be,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynPatternExpr {
    /// example: `1`
    Literal(LiteralData),
    /// example: `a`
    Ident {
        symbol_modifier_keyword_group: Option<EphemSymbolModifierTokenGroup>,
        ident_token: IdentToken,
    },
    /// example: `A::B`
    Entity(ItemPath),
    /// example: `(a, b)`
    Tuple {
        name: Option<ItemPath>,
        fields: SynPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        name: Option<ItemPath>,
        // todo: change to punctuated
        fields: SynPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: SynPatternExprIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident_token: IdentToken,
        asperand_token: AtToken,
        /// example: `1..9`
        src: SynPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: SynPatternExprIdx,
        dot_dot_token: DotDotToken,
        end: SynPatternExprIdx,
    },
}

pub type SynPatternExprArena = Arena<SynPatternExpr>;
pub type SynPatternExprIdx = ArenaIdx<SynPatternExpr>;
pub type SynPatternExprIdxRange = ArenaIdxRange<SynPatternExpr>;
pub type SynPatternExprMap<V> = ArenaMap<SynPatternExpr, V>;
pub type SynPatternExprOrderedMap<V> = ArenaOrderedMap<SynPatternExpr, V>;
