use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum SynPatternSymbol {
    Atom(SynPatternIdx),
}

impl SynPatternSymbol {
    pub(super) fn pattern_symbol_modifier(
        &self,
        pattern_expr_arena: &SynPatternExprArena,
    ) -> SvarModifier {
        match self {
            SynPatternSymbol::Atom(expr_idx) => match pattern_expr_arena[*expr_idx] {
                SynPatternExprData::Ident {
                    symbol_modifier_tokens: symbol_modifier_keyword_group,
                    ident_token: _,
                } => SvarModifier::new(symbol_modifier_keyword_group),
                _ => unreachable!(),
            },
        }
    }
}

pub type SynPatternSymbolArena = Arena<SynPatternSymbol>;
pub type SynPatternSymbolIdx = ArenaIdx<SynPatternSymbol>;
pub type SynPatternSymbolMap<V> = ArenaMap<SynPatternSymbol, V>;
pub type SynPatternSymbolOrderedMap<V> = ArenaOrderedMap<SynPatternSymbol, V>;
