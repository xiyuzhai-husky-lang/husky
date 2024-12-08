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
    fn build_ln_tactic_from_vd_stmt(&mut self, stmt: VdMirStmtIdx) -> Vec<LnMirTacticData> {
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
            VdMirStmtData::Then { prop } => self.build_ln_tactic_from_vd_then(prop),
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

    fn build_ln_tactic_from_vd_then(&mut self, prop: VdMirExprIdx) -> Vec<LnMirTacticData> {
        match self.expr_arena()[prop] {
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_separator_and_signature: Some((joined_separator, joined_signature)),
            } => self.build_then_nontrivial_chaining_separated_list(
                leader,
                followers,
                joined_separator,
                joined_signature,
            ),
            _ => {
                let ident = self.mangle_hypothesis();
                let ty = prop.to_lean(self);
                let tactics = self.alloc_tactics(vec![LnMirTacticData::Obvious]);
                let construction = self.alloc_expr(LnMirExprData::By { tactics });
                vec![LnMirTacticData::Have {
                    ident,
                    ty,
                    construction,
                }]
            }
        }
    }

    fn build_then_nontrivial_chaining_separated_list(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_separator: VdBaseSeparator,
        joined_signature: VdBaseSeparatorSignature,
    ) -> Vec<LnMirTacticData> {
        debug_assert!(followers.len() >= 2);
        let ident = self.mangle_hypothesis();
        // TODO: Maye use to_lean trait method?
        let tactic_data = LnMirTacticData::Calc {
            leader: leader.to_lean(self),
            followers: followers
                .iter()
                .copied()
                .map(|(func, expr)| {
                    let LnMirFunc::BinaryOpr {
                        opr, instantiation, ..
                    } = func.to_lean(self)
                    else {
                        todo!()
                    };
                    ((opr, instantiation), expr.to_lean(self))
                })
                .collect(),
        };
        vec![tactic_data]
    }
}
