use crate::*;
use husky_entity_path::PatternPath;
use husky_syn_expr::{LiteralData, SynPatternExpr, SynPatternExprIdx, SynPatternRoot};

#[derive(Debug, PartialEq, Eq)]
pub enum HirLazyPatternExpr {
    /// example: `1`
    /// todo: change this to primitive value data
    Literal(LiteralData),
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
pub type HirLazyPatternExprIdx = ArenaIdx<HirLazyPatternExpr>;
pub type HirLazyPatternExprIdxRange = ArenaIdxRange<HirLazyPatternExpr>;
pub type HirLazyPatternExprMap<V> = ArenaMap<HirLazyPatternExpr, V>;
pub type HirLazyPatternExprOrderedMap<V> = ArenaOrderedMap<HirLazyPatternExpr, V>;

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_pattern_expr(
        &mut self,
        syn_pattern_root: SynPatternRoot,
    ) -> HirLazyPatternExprIdx {
        let pattern_expr = self.new_pattern_expr_aux(syn_pattern_root.syn_pattern_expr_idx());
        self.alloc_pattern_expr(pattern_expr)
    }

    fn new_pattern_expr_aux(
        &mut self,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> HirLazyPatternExpr {
        match self.syn_expr_region_data()[syn_pattern_expr_idx] {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident {
                symbol_modifier_tokens: symbol_modifier_keyword_group,
                ident_token,
            } => HirLazyPatternExpr::Ident {
                // symbol_modifier: (),
                ident: ident_token.ident(),
            },
            SynPatternExpr::TypeVariant { .. } => todo!(),
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            SynPatternExpr::OneOf { ref options } => todo!(),
            SynPatternExpr::Binding {
                ident_token,
                asperand_token,
                src,
            } => todo!(),
            SynPatternExpr::Range {
                start,
                dot_dot_token,
                end,
            } => todo!(),
        }
    }
}
