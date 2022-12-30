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
pub enum BaseScopeResult {
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

// impl Expr {
//     fn base_scope_result(&self, arena: &ExprArena) -> BaseScopeResult {
//         todo!()
//         // match self {
//         //     ExprVariant::Atom(ref atom) => match atom {
//         //         AtomExpr::Literal(_) => BaseScopeResult::None,
//         //         AtomExpr::Symbol(symbol) => match symbol  {
//         //             Symbol::ModulePath(path) => BaseScopeResult::Some(path),
//         //             _ => BaseScopeResult::None,
//         //         },
//         //         AtomExpr::Unrecognized(_) | AtomExpr::Uncertain(_) => BaseScopeResult::Uncertain,
//         //     },
//         //     ExprVariant::Opn { opn_variant, opds } => match opn_variant {
//         //         RawOpnVariant::Binary(BinaryOpr::ScopeResolution) => {
//         //             arena[opds.start() + 1].base_scope_result()
//         //         }
//         //         RawOpnVariant::Binary(BinaryOpr::As) => todo!(),
//         //         _ => BaseScopeResult::None,
//         //     },
//         // }
//     }
// }

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

// impl Expr {
//     fn new(variant: Expr, range: TextRange, arena: &ExprArena) -> Self {
//         Self {
//             base_scope_result: variant.base_scope_result(arena),
//             variant,
//             range,
//         }
//     }

//     pub fn base_scope_result(&self) -> BaseScopeResult {
//         self.base_scope_result
//     }
// }
