use visored_sem_expr::expr::separated_list::VdSemSeparatedListFollower;

use super::*;

impl<'a> VdMirExprBuilder<'a> {
    pub(super) fn build_separated_list(
        &mut self,
        separator_class: VdSeparatorClass,
        leader: VdSemExprIdx,
        followers: &[VdSemSeparatedListFollower],
    ) -> VdMirExprData {
        match separator_class {
            VdSeparatorClass::Relation => self.build_chaining_separated_list(leader, followers),
            VdSeparatorClass::Comma => todo!(),
            VdSeparatorClass::Semicolon => todo!(),
            VdSeparatorClass::Space | VdSeparatorClass::Mul | VdSeparatorClass::Add => {
                self.build_folding_separated_list(leader, followers)
            }
        }
    }

    fn build_folding_separated_list(
        &mut self,
        leader: VdSemExprIdx,
        followers: &[VdSemSeparatedListFollower],
    ) -> VdMirExprData {
        VdMirExprData::FoldingSeparatedList {
            leader: leader.to_vd_mir(self),
            followers: followers
                .iter()
                .copied()
                .map(|follower| {
                    let VdSemSeparatedListFollowerDispatch::Folding {
                        base_separator,
                        signature,
                    } = follower.dispatch
                    else {
                        unreachable!()
                    };
                    let func = VdMirFunc::NormalBaseSeparator(signature);
                    (func, follower.expr.to_vd_mir(self))
                })
                .collect(),
        }
    }

    fn build_chaining_separated_list(
        &mut self,
        leader: VdSemExprIdx,
        followers: &[VdSemSeparatedListFollower],
    ) -> VdMirExprData {
        VdMirExprData::ChainingSeparatedList {
            leader: leader.to_vd_mir(self),
            followers: followers
                .iter()
                .copied()
                .map(|follower| match follower.dispatch {
                    VdSemSeparatedListFollowerDispatch::Chaining {
                        base_separator,
                        signature,
                    } => (
                        VdMirFunc::NormalBaseSeparator(signature),
                        follower.expr.to_vd_mir(self),
                    ),
                    VdSemSeparatedListFollowerDispatch::InSet { expr_ty } => {
                        (VdMirFunc::InSet, follower.expr.to_vd_mir(self))
                    }
                    VdSemSeparatedListFollowerDispatch::Folding {
                        base_separator,
                        signature,
                    } => unreachable!("follower.dispatch = {:?}", follower.dispatch),
                })
                .collect(),
        }
    }
}
