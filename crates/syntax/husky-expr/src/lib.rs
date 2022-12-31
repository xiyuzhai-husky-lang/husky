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
mod variable;

pub use atom::*;
pub use entity_path::*;
pub use error::*;
use husky_symbol::SymbolContext;
pub use parser::*;
pub use pattern::*;
pub use sheet::*;
pub use variable::*;

use husky_entity_path::EntityPath;
use husky_opn_syntax::*;
use husky_text::*;
use husky_token::*;
use husky_vfs::{ModulePath, VfsResult};
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
    Atom(AtomExpr),
    Opn { opn: Opn, opds: ExprIdxRange },
    Bracketed(ExprIdx),
}

impl From<AtomExpr> for Expr {
    fn from(atom: AtomExpr) -> Self {
        Expr::Atom(atom)
    }
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub(crate) type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprIdxRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;
