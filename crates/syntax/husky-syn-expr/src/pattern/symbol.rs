use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum PatternVariable {
    Atom(SynPatternIdx),
}

impl PatternVariable {
    pub(super) fn pattern_variable_modifier(
        &self,
        pattern_arena: &SynPatternArena,
    ) -> VariableModifier {
        match self {
            PatternVariable::Atom(expr_idx) => match pattern_arena[*expr_idx] {
                SynPatternData::Ident {
                    symbol_modifier_tokens: symbol_modifier_keyword_group,
                    ident_token: _,
                } => VariableModifier::new(symbol_modifier_keyword_group),
                _ => unreachable!(),
            },
        }
    }
}

pub type SynPatternSymbolArena = Arena<PatternVariable>;
pub type PatternVariableIdx = ArenaIdx<PatternVariable>;
pub type SynPatternSymbolMap<V> = ArenaMap<PatternVariable, V>;
pub type SynPatternSymbolOrderedMap<V> = ArenaOrderedMap<PatternVariable, V>;
