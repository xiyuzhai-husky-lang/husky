mod atom;
mod automata;
mod block;
mod error;
mod opn;
mod parser;
mod precedence;
#[cfg(test)]
mod tests;
mod variable;

pub use atom::*;
pub use automata::*;
pub use error::*;
use husky_entity_path::EntityPathItd;

pub use variable::*;

use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_symbol_syntax::SymbolKind;
use husky_text::*;

use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expr {
    pub variant: RawExprVariant,
    pub range: TextRange,
    base_scope_result: BaseScopeResult,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseScopeResult {
    None,
    Some(EntityPathItd),
    Uncertain,
}

impl TextRanged for Expr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Atom(RawAtomExpr),
    Opn {
        opn_variant: RawOpnVariant,
        opds: ExprRange,
    },
}

impl RawExprVariant {
    fn base_scope_result(&self, arena: &ExprArena) -> BaseScopeResult {
        match self {
            RawExprVariant::Atom(ref atom) => match atom {
                RawAtomExpr::Literal(_) => BaseScopeResult::None,
                RawAtomExpr::Symbol(symbol) => match symbol.kind {
                    SymbolKind::EntityPath(path) => BaseScopeResult::Some(path),
                    SymbolKind::Unrecognized => BaseScopeResult::Uncertain,
                    _ => BaseScopeResult::None,
                },
                RawAtomExpr::Uncertain => BaseScopeResult::Uncertain,
            },
            RawExprVariant::Opn { opn_variant, opds } => match opn_variant {
                RawOpnVariant::Binary(BinaryOpr::ScopeResolution) => {
                    arena[opds.start + 1].base_scope_result()
                }
                RawOpnVariant::Binary(BinaryOpr::As) => todo!(),
                _ => BaseScopeResult::None,
            },
        }
    }
}

impl From<RawAtomExpr> for RawExprVariant {
    fn from(atom: RawAtomExpr) -> Self {
        RawExprVariant::Atom(atom)
    }
}

use idx_arena::{map::ArenaMap, ArenaIdx, ArenaRange, IdxArena};

pub type ExprArena = IdxArena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprRange = ArenaRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;

impl Expr {
    fn new(variant: RawExprVariant, range: TextRange, arena: &ExprArena) -> Self {
        Self {
            base_scope_result: variant.base_scope_result(arena),
            variant,
            range,
        }
    }

    pub fn base_scope_result(&self) -> BaseScopeResult {
        self.base_scope_result
    }
}
