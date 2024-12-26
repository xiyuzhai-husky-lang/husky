use super::*;

impl<'a> VdLeanTranspilationBuilder<'a, Dense> {
    pub(super) fn build_ln_tactic_from_vd_have(
        &mut self,
        prop: VdMirExprIdx,
    ) -> Vec<LnMirTacticData> {
        match self.expr_arena()[prop] {
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_separator_and_signature: Some((joined_separator, joined_signature)),
            } => self.build_show_nontrivial_chaining_separated_list(
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

    fn build_show_nontrivial_chaining_separated_list(
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
        let ultimate_prop_function = VdMirFunc::NormalBaseSeparator(joined_signature).to_lean(self);
        let ultimate_prop_arguments = [leader, followers.last().unwrap().1].to_lean(self);
        let tactics = self.alloc_tactics(vec![tactic_data]);
        let construction = self.alloc_expr(LnMirExprData::By { tactics });
        vec![LnMirTacticData::Have {
            ident,
            ty: self.alloc_expr(LnMirExprData::Application {
                function: ultimate_prop_function,
                arguments: ultimate_prop_arguments,
            }),
            construction,
        }]
    }
}
