use crate::*;
use husky_syn_expr::{LiteralData, SynPatternExpr, SynPatternExprIdx};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirEagerPatternExpr {
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
        fields: HirEagerPatternExprIdxRange,
    },
    /// example: `C { .. }`
    Props {
        path: Option<PatternPath>,
        // todo: change to punctuated
        fields: HirEagerPatternExprIdxRange,
    },
    /// example: `A | B | C { .. }`
    OneOf {
        options: HirEagerPatternExprIdxRange,
    },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirEagerPatternExprIdx,
    },
    /// example: `1..9`
    Range {
        start: HirEagerPatternExprIdx,
        end: HirEagerPatternExprIdx,
    },
}

pub type HirEagerPatternExprArena = Arena<HirEagerPatternExpr>;
pub type HirEagerPatternExprIdx = ArenaIdx<HirEagerPatternExpr>;
pub type HirEagerPatternExprIdxRange = ArenaIdxRange<HirEagerPatternExpr>;
pub type HirEagerPatternExprMap<V> = ArenaMap<HirEagerPatternExpr, V>;
pub type HirEagerPatternExprOrderedMap<V> = ArenaOrderedMap<HirEagerPatternExpr, V>;

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_pattern_expr(
        &mut self,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> HirEagerPatternExprIdx {
        let pattern_expr = self.new_pattern_expr_aux(syn_pattern_expr_idx);
        self.alloc_pattern_expr(pattern_expr)
    }

    fn new_pattern_expr_aux(
        &mut self,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> HirEagerPatternExpr {
        match self.syn_expr_region_data()[syn_pattern_expr_idx] {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident {
                symbol_modifier_keyword_group,
                ident_token,
            } => HirEagerPatternExpr::Ident {
                // symbol_modifier: (),
                ident: ident_token.ident(),
            },
            SynPatternExpr::Entity(_) => todo!(),
            SynPatternExpr::Tuple { name, fields } => todo!(),
            SynPatternExpr::Props { name, fields } => todo!(),
            SynPatternExpr::OneOf { options } => todo!(),
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
