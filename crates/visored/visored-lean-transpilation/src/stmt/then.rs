use super::*;
use dictionary::func_key::VdFuncKeyTranslation;
use either::*;
use lean_mir_expr::{expr::application::LnMirFunc, tactic::LnMirTacticData};
use lean_opr::opr::binary::LnBinaryOpr;
use lean_term::instantiation::LnInstantiation;
use visored_mir_expr::expr::application::VdMirFunc;

impl<'a> VdLeanTranspilationBuilder<'a> {
    pub(super) fn build_then_stmt(&mut self, formula: VdMirExprIdx) -> LnItemDefnData {
        match self.expr_arena()[formula] {
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
            } if followers.len() >= 2 => {
                self.build_then_nontrivial_chaining_separated_list(leader, followers)
            }
            _ => {
                let symbol = self.mangle_hypothesis();
                LnItemDefnData::Def {
                    symbol,
                    ty: formula.to_lean(self),
                    // TODO: better??
                    body: self.sorry(),
                }
            }
        }
    }

    fn build_then_nontrivial_chaining_separated_list(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
    ) -> LnItemDefnData {
        debug_assert!(followers.len() >= 2);
        let symbol = self.mangle_hypothesis();
        // TODO: Maye use to_lean trait method?
        let mut cumulative_opr = None;
        let tactic_data = LnMirTacticData::Calc {
            leader: leader.to_lean(self),
            followers: followers
                .iter()
                .copied()
                .map(|(func, expr)| {
                    let opr = self.build_cumulative_binary_opr(cumulative_opr, func);
                    cumulative_opr = Some(opr);
                    (opr, expr.to_lean(self))
                })
                .collect(),
        };
        let Some((opr, instantiation)) = cumulative_opr else {
            todo!()
        };
        let return_ty_arguments = [leader, followers.last().unwrap().1].to_lean(self);
        LnItemDefnData::Def {
            symbol,
            ty: self.alloc_expr(LnMirExprData::Application {
                function: LnMirFunc::BinaryOpr { opr, instantiation },
                arguments: return_ty_arguments,
            }),
            body: self.alloc_tactics([tactic_data]).into(),
        }
    }

    fn build_cumulative_binary_opr(
        &mut self,
        prev: Option<(LnBinaryOpr, LnInstantiation)>,
        new_func: VdMirFunc,
    ) -> (LnBinaryOpr, LnInstantiation) {
        let new_func = match new_func.key_or_expr() {
            Left(func_key) => {
                let Some(translation) = self.dictionary().func_key_translation(func_key) else {
                    todo!("no translation for func key `{:?}`", func_key)
                };
                let VdFuncKeyTranslation::ChainingBinaryOpr(func_key) = *translation else {
                    todo!()
                };
                self.build_func_from_key(func_key)
            }
            Right(_) => todo!(),
        };
        let LnMirFunc::BinaryOpr {
            opr, instantiation, ..
        } = new_func
        else {
            todo!()
        };
        match prev {
            // TODO: use lisp-csv?
            Some((prev_opr, prev_instantiation)) => match (prev_opr, opr) {
                (prev_opr, opr) if prev_opr == opr => {
                    if prev_instantiation == instantiation {
                        (opr, instantiation)
                    } else {
                        todo!()
                    }
                }
                (prev_opr, opr) => todo!("case {} and {} not handled", prev_opr, opr),
            },
            None => (opr, instantiation),
        }
    }
}
