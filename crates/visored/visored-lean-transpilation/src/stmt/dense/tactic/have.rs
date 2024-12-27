use super::*;

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(super) fn build_ln_tactic_from_vd_have(&mut self, prop: VdMirExprIdx) -> LnMirTacticData {
        match self.expr_arena()[prop] {
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_separator_and_signature: Some((joined_separator, joined_signature)),
            } => self.build_have_nontrivial_chaining_separated_list(
                leader,
                followers,
                joined_separator,
                joined_signature,
            ),
            _ => {
                let ident = self.mangle_hypothesis();
                let ty = prop.to_lean(self);
                let construction_tactics = self.alloc_tactics(vec![LnMirTacticData::Obvious]);
                let construction = self.alloc_expr(LnMirExprData::By {
                    tactics: construction_tactics,
                });
                LnMirTacticData::Have {
                    ident,
                    ty,
                    construction,
                }
            }
        }
    }

    fn build_have_nontrivial_chaining_separated_list(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_separator: VdBaseSeparator,
        joined_signature: VdBaseSeparatorSignature,
    ) -> LnMirTacticData {
        let n = calc_number_of_foremost_equivalences(followers);
        match n {
            0 => self.build_have_nontrivial_chaining_separated_list_aux(
                leader,
                followers,
                joined_separator,
                joined_signature,
            ),
            n => self.build_have_nontrivial_chaining_separated_list_with_foremost_equivalences(
                leader,
                followers,
                joined_separator,
                joined_signature,
            ),
        }
    }

    fn build_have_nontrivial_chaining_separated_list_with_foremost_equivalences(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_separator: VdBaseSeparator,
        joined_signature: VdBaseSeparatorSignature,
    ) -> LnMirTacticData {
        todo!()
    }

    fn build_have_nontrivial_chaining_separated_list_aux(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_separator: VdBaseSeparator,
        joined_signature: VdBaseSeparatorSignature,
    ) -> LnMirTacticData {
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
        let ultimate_prop_function = VdMirFunc::NormalBaseSeparator(joined_signature).to_lean(self);
        let ultimate_prop_arguments = [leader, followers.last().unwrap().1].to_lean(self);
        let construction_tactics = self.alloc_tactics(vec![tactic_data]);
        let construction = self.alloc_expr(LnMirExprData::By {
            tactics: construction_tactics,
        });
        LnMirTacticData::Have {
            ident,
            ty: self.alloc_expr(LnMirExprData::Application {
                function: ultimate_prop_function,
                arguments: ultimate_prop_arguments,
            }),
            construction,
        }
    }
}

enum NontrivialChainingSeparatedListKind {
    WithoutForemostEquivalences,
    WithForemostEquivalences,
}

fn calc_number_of_foremost_equivalences(followers: &[(VdMirFunc, VdMirExprIdx)]) -> usize {
    followers
        .iter()
        .position(|(func, _)| !is_equivalence(func))
        .unwrap_or(followers.len())
}

fn is_equivalence(func: &VdMirFunc) -> bool {
    todo!()
}
