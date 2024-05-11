use crate::*;
use husky_entity_path::path::PatternPath;
use husky_syn_expr::{SynPatternData, SynPatternIdx, SynPatternRoot};
use husky_term_prelude::literal::Literal;
use idx_arena::ArenaRef;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirLazyPatternData {
    /// example: `1`
    /// todo: change this to primitive value data
    Literal(Literal),
    /// example: `a`
    Ident {
        // symbol_modifier: Option<EphemSymbolModifier>,
        ident: Ident,
    },
    /// example: `A::B`
    Unit(PatternPath),
    /// example: `(a, b)`
    Tuple {
        path: Option<PatternPath>,
        fields: HirLazyPatternIdxRange,
    },
    /// example: `C { .. }`
    Props {
        path: Option<PatternPath>,
        // todo: change to punctuated
        fields: HirLazyPatternIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: HirLazyPatternIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirLazyPatternIdx,
    },
    /// example: `1..9`
    Range {
        start: HirLazyPatternIdx,
        end: HirLazyPatternIdx,
    },
}

pub type HirLazyPatternArena = Arena<HirLazyPatternData>;
pub type HirLazyPatternArenaRef<'a> = ArenaRef<'a, HirLazyPatternData>;
pub type HirLazyPatternIdx = ArenaIdx<HirLazyPatternData>;
pub type HirLazyPatternIdxRange = ArenaIdxRange<HirLazyPatternData>;
pub type HirLazyPatternMap<V> = ArenaMap<HirLazyPatternData, V>;
pub type HirLazyPatternOrderedMap<V> = ArenaOrderedMap<HirLazyPatternData, V>;

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_pattern(
        &mut self,
        syn_pattern_root: impl Into<SynPatternRoot>,
    ) -> HirLazyPatternIdx {
        let syn_pattern_idx = syn_pattern_root.into().syn_pattern_idx();
        let pattern_data = self.new_pattern_aux(syn_pattern_idx);
        self.alloc_pattern(syn_pattern_idx, pattern_data)
    }

    fn new_pattern_aux(&mut self, syn_pattern_idx: SynPatternIdx) -> HirLazyPatternData {
        match self.syn_expr_region_data()[syn_pattern_idx] {
            SynPatternData::Literal { .. } => todo!(),
            SynPatternData::Ident {
                symbol_modifier_tokens: _symbol_modifier_keyword_group,
                ident_token,
            } => HirLazyPatternData::Ident {
                // symbol_modifier: (),
                ident: ident_token.ident(),
            },
            SynPatternData::UnitTypeVariant { .. } => todo!(),
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            SynPatternData::TupleTypeVariant { .. } => todo!(),
            SynPatternData::Props { name: _, fields: _ } => todo!(),
            SynPatternData::OneOf { options: _ } => todo!(),
            SynPatternData::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternData::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
