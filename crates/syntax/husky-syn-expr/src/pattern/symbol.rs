use idx_arena::ordered_map::ArenaOrderedMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynPatternSymbol {
    Atom(SynPatternExprIdx),
}

impl SynPatternSymbol {
    pub(super) fn pattern_symbol_modifier(
        &self,
        pattern_expr_arena: &SynPatternExprArena,
    ) -> EphemSymbolModifier {
        match self {
            SynPatternSymbol::Atom(expr_idx) => match pattern_expr_arena[*expr_idx] {
                SynPatternExpr::Ident {
                    symbol_modifier_keyword_group,
                    ident_token,
                } => EphemSymbolModifier::new(symbol_modifier_keyword_group),
                _ => unreachable!(),
            },
        }
    }
}

pub type SynPatternSymbolArena = Arena<SynPatternSymbol>;
pub type SynPatternSymbolIdx = ArenaIdx<SynPatternSymbol>;
pub type SynPatternSymbolMap<V> = ArenaMap<SynPatternSymbol, V>;
pub type SynPatternSymbolOrderedMap<V> = ArenaOrderedMap<SynPatternSymbol, V>;
