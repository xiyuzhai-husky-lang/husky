use husky_entity_path::EntityPath;
use husky_word::Identifier;
use idx_arena::{ArenaIdx, ArenaIdxRange};
use ordered_float::NotNan;

pub enum PatternExpr {
    Atom(PatternAtom),
    Opn {
        opn: PatternOpn,
        opds: PatternExprIdxRange,
    },
}

pub enum PatternAtom {
    Literal(LiteralData),
    Identifier(Identifier),
    Entity(EntityPath),
}

pub enum LiteralData {
    NotNanFloat,
    NotNanF32(NotNan<f32>),
}

pub enum PatternOpn {
    Tuple { name: Option<EntityPath> },
    Struct { name: Option<EntityPath> },
}

pub type PatternExprIdx = ArenaIdx<PatternExpr>;
pub type PatternExprIdxRange = ArenaIdxRange<PatternExpr>;
