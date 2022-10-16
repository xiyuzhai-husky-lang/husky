mod atom;
mod automata;
mod error;
mod opn;
mod parser;
mod precedence;
#[cfg(test)]
mod tests;
mod variant;

pub use atom::*;
pub use automata::*;
pub use error::*;
use husky_term::TermPtr;
use optional::Optioned;
pub use variant::*;

use husky_opn_syntax::*;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_symbol_syntax::{Symbol, SymbolKind};
use husky_text::*;
use husky_token::{Token, TokenKind};
use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawExpr {
    pub variant: RawExprVariant,
    pub range: TextRange,
    opt_scope: Optioned<TermPtr>,
}

impl TextRanged for RawExpr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Atom(RawAtom),
    Bracketed(RawExprIdx),
    Opn {
        opn_variant: RawOpnVariant,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(RangedCustomIdentifier, Option<RawExprIdx>)>,
        RawExprIdx,
    ),
}

impl RawExprVariant {
    fn opt_scope(&self, arena: &RawExprArena) -> Optioned<TermPtr> {
        match self {
            RawExprVariant::Atom(ref atom) => match atom {
                RawAtom::Literal(_) => Optioned::none(),
                RawAtom::Symbol(symbol) => match symbol.kind {
                    SymbolKind::EntityPath(_) => todo!(),
                    _ => Optioned::none(),
                },
            },
            RawExprVariant::Bracketed(_) => todo!(),
            RawExprVariant::Opn { opn_variant, opds } => match opn_variant {
                RawOpnVariant::Binary(BinaryOpr::ScopeResolution) => todo!(),
                RawOpnVariant::Binary(BinaryOpr::As) => todo!(),
                _ => Optioned::none(),
            },
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }
}

impl From<RawAtom> for RawExprVariant {
    fn from(atom: RawAtom) -> Self {
        RawExprVariant::Atom(atom)
    }
}

use idx_arena::{map::ArenaMap, ArenaIdx, ArenaRange, IdxArena};

pub type RawExprArena = IdxArena<RawExpr>;
pub type RawExprIdx = ArenaIdx<RawExpr>;
pub type RawExprRange = ArenaRange<RawExpr>;
pub type RawExprMap<V> = ArenaMap<RawExpr, V>;

impl RawExpr {
    fn new(variant: RawExprVariant, range: TextRange, arena: &RawExprArena) -> Self {
        Self {
            opt_scope: variant.opt_scope(arena),
            variant,
            range,
        }
    }

    pub fn opt_scope(&self) -> Optioned<TermPtr> {
        self.opt_scope
    }
}
