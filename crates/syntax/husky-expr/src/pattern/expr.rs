use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum PatternExpr {
    /// example: `1`
    Literal(LiteralData),
    /// example: `a`
    Identifier {
        ident_token: IdentifierToken,
        liason: PatternLiason,
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
        ident_token: IdentifierToken,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternLiason {
    None,
    Mut,
}
