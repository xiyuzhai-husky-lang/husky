use husky_entity_path::EntityPath;
use husky_word::Identifier;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;

#[derive(Debug, PartialEq, Eq)]
pub enum PatternExpr {
    Atom(PatternAtom),
    Opn {
        opn: PatternOpn,
        opds: PatternExprIdxRange,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternAtom {
    Literal(LiteralData),
    Identifier(Identifier),
    Entity(EntityPath),
}

#[derive(Debug, PartialEq, Eq)]
pub enum LiteralData {
    NotNanFloat,
    NotNanF32(NotNan<f32>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PatternOpn {
    Tuple { name: Option<EntityPath> },
    Struct { name: Option<EntityPath> },
}

pub(crate) type PatternExprArena = Arena<PatternExpr>;
pub type PatternExprIdx = ArenaIdx<PatternExpr>;
pub type PatternExprIdxRange = ArenaIdxRange<PatternExpr>;
