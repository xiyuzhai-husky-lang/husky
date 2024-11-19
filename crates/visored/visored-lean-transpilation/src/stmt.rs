use crate::*;
use lean_mir_expr::item_defn::{LnItemDefnData, LnItemDefnIdxRange, LnMirItemDefnGroupMeta};
use ty::VdTypeLeanTranspilation;
use visored_mir_expr::{
    pattern::VdMirPattern,
    stmt::{block::VdMirBlockMeta, VdMirStmtData, VdMirStmtIdx, VdMirStmtIdxRange},
};
use visored_term::ty::VdType;

impl VdTranspileToLean<LnItemDefnIdxRange> for VdMirStmtIdxRange {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> LnItemDefnIdxRange {
        let item_defns: Vec<_> = self
            .into_iter()
            .map(|stmt| builder.build_ln_item_defn_from_vd_stmt(stmt))
            .collect();
        builder.alloc_item_defns(item_defns)
    }
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub(crate) fn build_ln_item_defn_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> LnItemDefnData {
        let db = self.db();
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => {
                let defns = stmts.to_lean(self);
                let meta = match meta {
                    VdMirBlockMeta::Paragraph => LnMirItemDefnGroupMeta::Paragraph,
                    VdMirBlockMeta::Sentence => LnMirItemDefnGroupMeta::Sentence,
                    VdMirBlockMeta::Division(vd_division_level) => LnMirItemDefnGroupMeta::Division,
                    VdMirBlockMeta::Environment(lx_environment_path) => {
                        LnMirItemDefnGroupMeta::Environment
                    }
                };
                LnItemDefnData::Group { defns, meta }
            }
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => {
                self.build_ln_item_from_vd_let_placeholder_stmt(pattern, ty)
            }
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => todo!(),
            VdMirStmtData::Then { formula } => {
                let symbol = self.mangle_hypothesis(db);
                LnItemDefnData::Def {
                    symbol,
                    ty: formula.to_lean(self),
                    // TODO: better??
                    body: self.sorry(),
                }
            }
        }
    }

    fn build_ln_item_from_vd_let_placeholder_stmt(
        &mut self,
        pattern: &VdMirPattern,
        ty: VdType,
    ) -> LnItemDefnData {
        match *pattern {
            VdMirPattern::Letter {
                symbol_local_defn, ..
            } => {
                let ident = self.mangle_symbol(symbol_local_defn);
                match ty.to_lean(self) {
                    VdTypeLeanTranspilation::Type(ty) => {
                        LnItemDefnData::Variable { symbol: ident, ty }
                    }
                }
            }
        }
    }
}
