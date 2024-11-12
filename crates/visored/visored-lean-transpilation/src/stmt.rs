use crate::*;
use lean_hir_expr::item_defn::{LnItemDefnData, LnItemDefnIdxRange};
use visored_hir_expr::stmt::{
    block::VdHirBlockMeta, VdHirStmtData, VdHirStmtIdx, VdHirStmtIdxRange,
};

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
                match meta {
                    VdHirBlockMeta::Paragraph => todo!(),
                    VdHirBlockMeta::Sentence => todo!(),
                }
            }
            VdHirStmtData::Let => todo!(),
        }
    }
}
