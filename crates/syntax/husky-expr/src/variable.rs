use crate::*;

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum RawVariableKind {
//     Normal { init_range: TextRange },
//     ThisField,
//     Unrecognized,
//     Block,
// }

pub type VariableArena = Arena<Variable>;
pub type VariableIdx = ArenaIdx<Variable>;
pub type VariableIdxRange = ArenaIdxRange<Variable>;

pub enum Variable {
    Let { pattern_symbol: PatternSymbolIdx },
}

pub enum Prevariable {}
