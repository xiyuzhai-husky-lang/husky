use crate::*;
use lean_hir_expr::item_defn::{LnHirItemDefnGroupMeta, LnItemDefnData, LnItemDefnIdxRange};
use ty::VdZfcTypeLeanTranspilation;
use visored_hir_expr::{
    pattern::VdHirPattern,
    stmt::{block::VdHirBlockMeta, VdHirStmtData, VdHirStmtIdx, VdHirStmtIdxRange},
};
use visored_zfc_ty::ty::VdZfcType;

impl VdTranspileToLean<LnItemDefnIdxRange> for VdHirStmtIdxRange {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> LnItemDefnIdxRange {
        let item_defns: Vec<_> = self
            .into_iter()
            .map(|stmt| builder.build_ln_item_defn_from_vd_stmt(stmt))
            .collect();
        builder.alloc_item_defns(item_defns)
    }
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub(crate) fn build_ln_item_defn_from_vd_stmt(&mut self, stmt: VdHirStmtIdx) -> LnItemDefnData {
        match self.stmt_arena()[stmt] {
            VdHirStmtData::Expression(arena_idx) => todo!(),
            VdHirStmtData::Block { stmts, ref meta } => {
                let defns = stmts.to_lean(self);
                let meta = match meta {
                    VdHirBlockMeta::Paragraph => LnHirItemDefnGroupMeta::Paragraph,
                    VdHirBlockMeta::Sentence => LnHirItemDefnGroupMeta::Sentence,
                };
                LnItemDefnData::Group { defns, meta }
            }
            VdHirStmtData::LetPlaceholder { ref pattern, ty } => {
                self.build_ln_item_from_vd_let_placeholder_stmt(pattern, ty)
            }
            VdHirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => todo!(),
        }
    }

    fn build_ln_item_from_vd_let_placeholder_stmt(
        &mut self,
        pattern: &VdHirPattern,
        ty: VdZfcType,
    ) -> LnItemDefnData {
        match *pattern {
            VdHirPattern::Letter {
                symbol_local_defn, ..
            } => {
                let ident = self.mangled_symbol(symbol_local_defn);
                match ty.to_lean(self) {
                    VdZfcTypeLeanTranspilation::Type(ty) => {
                        LnItemDefnData::Variable { symbol: ident, ty }
                    }
                }
            }
        }
    }
}
