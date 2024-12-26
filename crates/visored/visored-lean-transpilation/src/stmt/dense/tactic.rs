mod have;
mod show;

use super::*;
use lean_mir_expr::{
    expr::application::LnMirFunc,
    tactic::{LnMirTacticData, LnMirTacticIdxRange},
};
use visored_mir_expr::expr::application::VdMirFunc;
use visored_opr::separator::VdBaseSeparator;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

impl VdTranspileToLean<Dense, LnMirTacticIdxRange> for VdMirStmtIdxRange {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder<Dense>) -> LnMirTacticIdxRange {
        let tactics: Vec<LnMirTacticData> = self
            .into_iter()
            .flat_map(|stmt| builder.build_ln_tactic_from_vd_stmt(stmt))
            .collect();
        builder.alloc_tactics(tactics)
    }
}

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(crate) fn build_ln_tactic_from_vd_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
    ) -> Vec<LnMirTacticData> {
        let db = self.db();
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => match meta {
                VdMirBlockMeta::Paragraph => self.build_ln_tactic_from_vd_paragraph(stmts),
                VdMirBlockMeta::Sentence => self.build_ln_tactic_from_vd_sentence(stmts),
                VdMirBlockMeta::Environment(lx_environment_path, _, vd_module_path) => todo!(),
                VdMirBlockMeta::Division(vd_division_level, vd_module_path) => todo!(),
            },
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => {
                vec![]
            }
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => vec![LnMirTacticData::Have {
                ident: todo!(),
                ty: todo!(),
                construction: todo!(),
            }],
            VdMirStmtData::Have { prop } => self.build_ln_tactic_from_vd_have(prop),
            VdMirStmtData::Show { prop } => self.build_ln_tactic_from_vd_show(prop),
        }
    }

    fn build_ln_tactic_from_vd_paragraph(
        &mut self,
        stmts: VdMirStmtIdxRange,
    ) -> Vec<LnMirTacticData> {
        stmts
            .into_iter()
            .flat_map(|stmt| self.build_ln_tactic_from_vd_stmt(stmt))
            .collect()
    }

    fn build_ln_tactic_from_vd_sentence(
        &mut self,
        stmts: VdMirStmtIdxRange,
    ) -> Vec<LnMirTacticData> {
        match stmts.len() {
            0 => todo!(),
            1 => self.build_ln_tactic_from_vd_stmt(stmts.first().unwrap()),
            _ => todo!(),
        }
    }
}
