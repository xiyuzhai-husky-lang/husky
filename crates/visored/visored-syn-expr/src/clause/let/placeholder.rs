use super::*;
use pattern::VdSynPattern;
use symbol::local_defn::{VdSynSymbolLocalDefnBody, VdSynSymbolLocalDefnHead};

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynLetPlaceholderResolution {
    pattern: VdSynPattern,
    ty: VdSynLetClausePlaceholderType,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynLetClausePlaceholderType {
    Expr(VdSynExprIdx),
}

impl<'db> VdSynExprBuilder<'db> {
    pub fn build_let_placeholder_resolution(
        &self,
        pattern: VdSynExprIdx,
        ty: VdSynLetClausePlaceholderType,
    ) -> VdSynLetPlaceholderResolution {
        VdSynLetPlaceholderResolution {
            pattern: self.build_pattern(pattern),
            ty,
        }
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub(crate) fn build_let_placeholder_resolution(
        &mut self,
        clause: VdSynClauseIdx,
        resolution: &VdSynLetPlaceholderResolution,
    ) {
        // Order matters!
        self.build_let_clause_placeholder_ty(resolution.ty);
        match resolution.pattern {
            VdSynPattern::Letter(token_idx_range, letter) => {
                self.define_symbol(
                    VdSynSymbolLocalDefnHead::Letter {
                        token_idx_range,
                        letter,
                    },
                    VdSynSymbolLocalDefnBody::Placeholder,
                    clause.into(),
                );
            }
        }
    }

    fn build_let_clause_placeholder_ty(&mut self, ty: VdSynLetClausePlaceholderType) {
        match ty {
            VdSynLetClausePlaceholderType::Expr(expr) => self.build_expr(expr),
        }
    }
}
