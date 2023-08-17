use husky_syn_expr::{SynPatternExpr, SynPatternExprIdx};

use crate::*;

#[salsa::interned(db = HirPatternDb, jar = HirPatternJar, constructor = new_inner)]
pub struct HirPattern {
    #[return_ref]
    data: HirPatternData,
    ty: HirType,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum HirPatternData {
    /// example: `1`
    Literal(/* todo */),
    /// example: `a`
    Ident {
        symbol_modifier: Option<SymbolModifier>,
        ident: Ident,
    },
    /// example: `A::B`
    Entity(ItemPath),
    /// example: `(a, b)`
    Tuple {
        name: Option<ItemPath>,
        fields: SmallVec<[HirPattern; 2]>,
    },
    /// example: `C { .. }`
    Props {
        name: Option<ItemPath>,
        fields: SmallVec<[HirPattern; 4]>,
    },
    /// example: `A | B | C { .. }`
    OneOf { options: SmallVec<[HirPattern; 4]> },
    /// example: `x @ 1..9`
    Binding {
        ident: Ident,
        /// example: `1..9`
        src: HirPattern,
    },
    /// example: `1..9`
    Range { start: HirPattern, end: HirPattern },
}

impl HirPattern {
    pub fn new(builder: &impl BuildHirPattern, pattern_expr_idx: SynPatternExprIdx) -> Self {
        match builder.syn_pattern_expr_arena()[pattern_expr_idx] {
            SynPatternExpr::Literal(_) => todo!(),
            SynPatternExpr::Ident {
                symbol_modifier_keyword_group,
                ident_token,
            } => todo!(),
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
