use crate::*;
use husky_entity_path::PatternPath;
use husky_syn_expr::{SynPatternExpr, SynPatternExprIdx, SynPatternExprRoot};
use husky_term_prelude::TermLiteral;
use idx_arena::ArenaRef;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirLazyPatternExpr {
    /// example: `1`
    /// todo: change this to primitive value data
    Literal(TermLiteral),
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
        fields: HirLazyPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        path: Option<PatternPath>,
        // todo: change to punctuated
        fields: HirLazyPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: HirLazyPatternExprIdxRange },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirLazyPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: HirLazyPatternExprIdx,
        end: HirLazyPatternExprIdx,
    },
}

pub type HirLazyPatternExprArena = Arena<HirLazyPatternExpr>;
pub type HirLazyPatternExprArenaRef<'a> = ArenaRef<'a, HirLazyPatternExpr>;
pub type HirLazyPatternExprIdx = ArenaIdx<HirLazyPatternExpr>;
pub type HirLazyPatternExprIdxRange = ArenaIdxRange<HirLazyPatternExpr>;
pub type HirLazyPatternExprMap<V> = ArenaMap<HirLazyPatternExpr, V>;
pub type HirLazyPatternExprOrderedMap<V> = ArenaOrderedMap<HirLazyPatternExpr, V>;

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_pattern_expr(
        &mut self,
        syn_pattern_root: impl Into<SynPatternExprRoot>,
    ) -> HirLazyPatternExprIdx {
        let pattern_expr =
            self.new_pattern_expr_aux(syn_pattern_root.into().syn_pattern_expr_idx());
        self.alloc_pattern_expr(pattern_expr)
    }

    fn new_pattern_expr_aux(
        &mut self,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> HirLazyPatternExpr {
        match self.syn_expr_region_data()[syn_pattern_expr_idx] {
            SynPatternExpr::Literal { .. } => todo!(),
            SynPatternExpr::Ident {
                symbol_modifier_tokens: _symbol_modifier_keyword_group,
                ident_token,
            } => HirLazyPatternExpr::Ident {
                // symbol_modifier: (),
                ident: ident_token.ident(),
            },
            SynPatternExpr::TypeVariantUnit { .. } => todo!(),
            SynPatternExpr::Tuple { name: _, fields: _ } => todo!(),
            SynPatternExpr::Props { name: _, fields: _ } => todo!(),
            SynPatternExpr::OneOf { options: _ } => todo!(),
            SynPatternExpr::Binding {
                ident_token: _,
                asperand_token: _,
                src: _,
            } => todo!(),
            SynPatternExpr::Range {
                start: _,
                dot_dot_token: _,
                end: _,
            } => todo!(),
        }
    }
}
