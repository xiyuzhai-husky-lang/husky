use super::*;
use crate::scheme::VdLeanTranspilationDenseScheme as Dense;
use lean_mir_expr::tactic::{LnMirTacticData, LnMirTacticIdxRange};

impl IsSchemeForTranspileVdStmtsToLnDefns for Dense {
    fn transpile_vd_stmts_to_ln_defns(
        builder: &mut VdLeanTranspilationBuilder<Self>,
        stmts: VdMirStmtIdxRange,
    ) -> LnItemDefnIdxRange {
        todo!()
    }
}

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
    fn build_ln_tactic_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> Vec<LnMirTacticData> {
        let db = self.db();
        match self.stmt_arena()[stmt] {
            VdMirStmtData::Block { stmts, ref meta } => match meta {
                VdMirBlockMeta::Paragraph => todo!(),
                VdMirBlockMeta::Sentence => todo!(),
                VdMirBlockMeta::Environment(lx_environment_path, vd_module_path) => todo!(),
                VdMirBlockMeta::Division(vd_division_level, vd_module_path) => todo!(),
            },
            VdMirStmtData::LetPlaceholder { ref pattern, ty } => {
                vec![LnMirTacticData::Intro {}]
            }
            VdMirStmtData::LetAssigned {
                ref pattern,
                assignment,
            } => vec![LnMirTacticData::Have {
                ident: todo!(),
                ty: todo!(),
                construction: todo!(),
            }],
            VdMirStmtData::Then { prop } => todo!(),
        }
    }
}
