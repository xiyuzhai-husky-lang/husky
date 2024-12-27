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
        let mut tactics = Vec::new();
        builder.build_ln_tactics_from_vd_stmts(self, &mut tactics);
        tactics.push(LnMirTacticData::Obvious);
        builder.alloc_tactics(tactics)
    }
}

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(crate) fn build_ln_tactics_from_vd_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        tactics: &mut Vec<LnMirTacticData>,
    ) {
        let db = self.db();
        let Some((stmt, following_stmts)) = stmts.first_and_others() else {
            return;
        };
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => {
                match meta {
                    VdMirBlockMeta::Paragraph => {
                        self.build_ln_tactic_from_vd_paragraph(stmts, tactics)
                    }
                    VdMirBlockMeta::Sentence => {
                        self.build_ln_tactic_from_vd_sentence(stmts, tactics)
                    }
                    VdMirBlockMeta::Environment(lx_environment_path, _, vd_module_path) => todo!(),
                    VdMirBlockMeta::Division(vd_division_level, vd_module_path) => todo!(),
                }
                self.build_ln_tactics_from_vd_stmts(following_stmts, tactics);
            }
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => {
                // Empty - no tactics to add
                // TODO: maybe intro in certain cases?
                self.build_ln_tactics_from_vd_stmts(following_stmts, tactics);
            }
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => {
                tactics.push(LnMirTacticData::Have {
                    ident: todo!(),
                    ty: todo!(),
                    construction: todo!(),
                });
                self.build_ln_tactics_from_vd_stmts(following_stmts, tactics);
            }
            VdMirStmtData::Goal { prop } => {
                self.build_ln_tactics_from_vd_stmts(following_stmts, tactics);
            }
            VdMirStmtData::Have { prop } => {
                tactics.push(self.build_ln_tactic_from_vd_have(prop));
                self.build_ln_tactics_from_vd_stmts(following_stmts, tactics);
            }
            VdMirStmtData::Show { prop } => {
                // Here, we also provide the following stmts to build the tactic.
                tactics.push(self.build_ln_tactic_from_vd_show(prop, following_stmts));
            }
        }
    }

    fn build_ln_tactic_from_vd_paragraph(
        &mut self,
        stmts: VdMirStmtIdxRange,
        tactics: &mut Vec<LnMirTacticData>,
    ) {
        self.build_ln_tactics_from_vd_stmts(stmts, tactics);
    }

    fn build_ln_tactic_from_vd_sentence(
        &mut self,
        stmts: VdMirStmtIdxRange,
        tactics: &mut Vec<LnMirTacticData>,
    ) {
        self.build_ln_tactics_from_vd_stmts(stmts, tactics)
    }
}
