mod atom;
mod automata;
mod error;
mod opn;
mod precedence;
mod stmt;
#[cfg(test)]
mod tests;
mod variable;

pub use atom::*;
pub use automata::*;
pub use error::*;
pub use variable::*;

use husky_entity_path::EntityPath;
use husky_opn_syntax::*;
use husky_symbol_syntax::SymbolKind;
use husky_text::*;
use husky_token::*;
use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expr {
    pub variant: ExprVariant,
    pub range: TextRange,
    base_scope_result: BaseScopeResult,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseScopeResult {
    None,
    Some(EntityPath),
    Uncertain,
}

impl HasTextRange for Expr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprVariant {
    Atom(AtomExpr),
    Opn {
        opn_variant: RawOpnVariant,
        opds: ExprRange,
    },
}

impl ExprVariant {
    fn base_scope_result(&self, arena: &ExprArena) -> BaseScopeResult {
        todo!()
        // match self {
        //     ExprVariant::Atom(ref atom) => match atom {
        //         AtomExpr::Literal(_) => BaseScopeResult::None,
        //         AtomExpr::Symbol(symbol) => match symbol.kind {
        //             SymbolKind::ModulePath(path) => BaseScopeResult::Some(path),
        //             _ => BaseScopeResult::None,
        //         },
        //         AtomExpr::Unrecognized(_) | AtomExpr::Uncertain(_) => BaseScopeResult::Uncertain,
        //     },
        //     ExprVariant::Opn { opn_variant, opds } => match opn_variant {
        //         RawOpnVariant::Binary(BinaryOpr::ScopeResolution) => {
        //             arena[opds.start() + 1].base_scope_result()
        //         }
        //         RawOpnVariant::Binary(BinaryOpr::As) => todo!(),
        //         _ => BaseScopeResult::None,
        //     },
        // }
    }
}

impl From<AtomExpr> for ExprVariant {
    fn from(atom: AtomExpr) -> Self {
        ExprVariant::Atom(atom)
    }
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;

impl Expr {
    fn new(variant: ExprVariant, range: TextRange, arena: &ExprArena) -> Self {
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
