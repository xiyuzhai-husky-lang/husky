#![feature(trait_upcasting)]
mod atom;
mod entity_path;
mod error;
mod opn;
mod parser;
mod pattern;
mod precedence;
mod sheet;
mod stmt;
#[cfg(test)]
mod tests;
mod variable;

pub use atom::*;
pub use entity_path::*;
pub use error::*;
pub use opn::*;
pub use parser::*;
pub use pattern::*;
pub use sheet::*;
pub use variable::*;

use husky_entity_path::EntityPath;
use husky_opn_syntax::*;
use husky_symbol::VariableIdx;
use husky_text::*;
use husky_token::*;
use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal(TokenIdx),
    EntityPath(EntityPathExprIdx),
    Variable {
        token_idx: TokenIdx,
        variable_idx: VariableIdx,
    },
    Uncertain(Identifier),
    Unrecognized(Identifier),
    Opn {
        opn: Opn,
        opds: ExprIdxRange,
    },
    MethodCall {
        this_expr: ExprIdx,
        arguments: ExprIdxRange,
        lpar_token_idx: TokenIdx,
        rpar_token_idx: TokenIdx,
    },
    Bracketed(ExprIdx),
    Err(ExprError),
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub(crate) type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprIdxRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;
