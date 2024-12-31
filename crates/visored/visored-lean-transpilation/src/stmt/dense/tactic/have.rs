use super::*;
use visored_mir_expr::hint::VdMirHintIdxRange;

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(super) fn build_ln_tactic_from_vd_have(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
    ) -> LnMirTacticData {
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
                let tactics = self.build_have_tactics(stmt);
                let construction = self.alloc_expr(LnMirExprData::By { tactics });
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
                n,
            ),
        }
    }

    fn build_have_nontrivial_chaining_separated_list_with_foremost_equivalences(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_separator: VdBaseSeparator,
        joined_signature: VdBaseSeparatorSignature,
        number_of_foremost_equivalences: usize,
    ) -> LnMirTacticData {
        let foremost_equivalences = &followers[..number_of_foremost_equivalences];
        let non_foremost_equivalences = &followers[number_of_foremost_equivalences..];
        let reverse_leader = foremost_equivalences.last().unwrap().1;
        let reverse_followers: Vec<_> = (0..number_of_foremost_equivalences)
            .into_iter()
            .map(|i| {
                (
                    foremost_equivalences[number_of_foremost_equivalences - 1 - i].0,
                    if i < number_of_foremost_equivalences - 1 {
                        foremost_equivalences[number_of_foremost_equivalences - 2 - i].1
                    } else {
                        leader
                    },
                )
            })
            .chain(non_foremost_equivalences.iter().copied())
            .collect();
        assert!(followers.len() == reverse_followers.len());
        let forward_tactic_data = self.build_have_nontrivial_chaining_separated_list_aux(
            leader,
            &followers,
            joined_separator,
            joined_signature,
        );
        let backward_tactic_data = self.build_have_nontrivial_chaining_separated_list_aux(
            reverse_leader,
            &reverse_followers,
            joined_separator,
            joined_signature,
        );
        LnMirTacticData::First {
            arms: self.alloc_tactics([forward_tactic_data, backward_tactic_data]),
        }
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
    match func {
        VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
        VdMirFunc::NormalBaseSeparator(signature) => match signature.opr() {
            VdBaseSeparator::Space => todo!(),
            VdBaseSeparator::Comma => todo!(),
            VdBaseSeparator::Semicolon => todo!(),
            VdBaseSeparator::Add => todo!(),
            VdBaseSeparator::Mul => todo!(),
            VdBaseSeparator::Dot => todo!(),
            VdBaseSeparator::Eq => true,
            VdBaseSeparator::Ne => false,
            VdBaseSeparator::Lt => false,
            VdBaseSeparator::Gt => false,
            VdBaseSeparator::Le => false,
            VdBaseSeparator::Ge => false,
            VdBaseSeparator::Subset => false,
            VdBaseSeparator::Supset => false,
            VdBaseSeparator::Subseteq => false,
            VdBaseSeparator::Supseteq => false,
            VdBaseSeparator::Subseteqq => false,
            VdBaseSeparator::Supseteqq => false,
            VdBaseSeparator::Subsetneq => false,
            VdBaseSeparator::Supsetneq => false,
            VdBaseSeparator::In => false,
            VdBaseSeparator::Notin => false,
            VdBaseSeparator::Times => todo!(),
            VdBaseSeparator::Otimes => todo!(),
        },
        VdMirFunc::NormalBaseBinaryOpr(signature) => todo!(),
        VdMirFunc::Power(signature) => todo!(),
        VdMirFunc::InSet => todo!(),
        VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
        VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
    }
}
