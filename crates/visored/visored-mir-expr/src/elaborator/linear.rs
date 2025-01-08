use super::*;
use crate::stmt::{VdMirStmtData, VdMirStmtMap};
use expr::{application::VdMirFunc, VdMirExprData, VdMirExprIdxRange};
use hint::VdMirHintIdx;
use hypothesis::{
    chunk::VdMirHypothesisChunk, construction::VdMirHypothesisConstruction, VdMirHypothesisIdx,
};
use smallvec::SmallVec;
use smallvec::ToSmallVec;
use visored_mir_opr::{
    opr::{binary::VdMirBaseBinaryOpr, prefix::VdMirBasePrefixOpr},
    separator::VdMirBaseSeparator,
};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

#[derive(Default)]
pub struct VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    inner: Inner,
}

pub trait IsVdMirSequentialElaboratorInner {
    type HypothesisIdx: std::fmt::Debug + Eq;
    type Contradiction: std::fmt::Debug;

    fn elaborate_let_placeholder_stmt(&mut self) -> Result<(), Self::Contradiction>;

    fn elaborate_assume_stmt(
        &mut self,
        prop: VdMirExprIdx,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction>;

    fn elaborate_let_assigned_stmt(&mut self) -> Result<(), Self::Contradiction>;

    fn elaborate_goal_stmt(&mut self) -> Result<(), Self::Contradiction>;

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction>;

    fn elaborate_show_stmt(&mut self) -> Result<Self::HypothesisIdx, Self::Contradiction>;

    fn elaborate_qed_stmt(&mut self) -> Result<Self::HypothesisIdx, Self::Contradiction>;

    fn elaborate_field_div_expr(
        &mut self,
        divisor: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction>;

    fn elaborate_folding_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
    );

    fn elaborate_chaining_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_signature: Option<VdBaseSeparatorSignature>,
    );

    fn cache_expr(&mut self, expr: VdMirExprIdx, region_data: VdMirExprRegionDataRef);

    fn transcribe_explicit_hypothesis(
        &mut self,
        hypothesis: Self::HypothesisIdx,
        expr: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx;
}

impl IsVdMirSequentialElaboratorInner for () {
    type HypothesisIdx = ();
    type Contradiction = ();

    fn elaborate_let_assigned_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_let_placeholder_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_assume_stmt(&mut self, prop: VdMirExprIdx) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_goal_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_show_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_qed_stmt(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn elaborate_field_div_expr(
        &mut self,
        divisor: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> Result<Self::HypothesisIdx, Self::Contradiction> {
        Ok(())
    }

    fn elaborate_folding_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
    ) {
        ()
    }

    fn elaborate_chaining_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_signature: Option<VdBaseSeparatorSignature>,
    ) {
        ()
    }

    fn cache_expr(&mut self, expr: VdMirExprIdx, region_data: VdMirExprRegionDataRef) {
        ()
    }

    #[track_caller]
    fn transcribe_explicit_hypothesis(
        &mut self,
        hypothesis: (),
        expr: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        hypothesis_constructor.construct_new_hypothesis(expr, VdMirHypothesisConstruction::Sorry)
    }
}

impl<Inner> VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    pub fn new(inner: Inner) -> Self {
        Self { inner }
    }
}

impl<Inner> IsVdMirTacticElaborator for VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    // # elaborate
    fn elaborate_stmts_ext(
        mut self,
        stmts: VdMirStmtIdxRange,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        self.elaborate_stmts(stmts, hypothesis_constructor);
    }

    fn elaborate_stmt_ext(
        mut self,
        stmt: VdMirStmtIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        self.elaborate_stmt(stmt, hypothesis_constructor);
    }

    fn elaborate_expr_ext(
        mut self,
        expr: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        self.elaborate_expr(expr, hypothesis_constructor);
    }
}

impl<Inner> VdMirSequentialElaborator<Inner>
where
    Inner: IsVdMirSequentialElaboratorInner,
{
    fn elaborate_stmts(
        &mut self,
        stmts: VdMirStmtIdxRange,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        for stmt in stmts {
            self.elaborate_stmt(stmt, hypothesis_constructor);
        }
    }

    fn elaborate_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        match *hypothesis_constructor.stmt_arena()[stmt].data() {
            VdMirStmtData::Block { stmts, .. } => {
                self.elaborate_stmts(stmts, hypothesis_constructor)
            }
            VdMirStmtData::LetPlaceholder { .. } => {
                self.inner
                    .elaborate_let_placeholder_stmt()
                    .expect("handle contradiction");
            }
            VdMirStmtData::Assume { prop, .. } => {
                self.elaborate_expr(prop, hypothesis_constructor);
                let hypothesis = self
                    .inner
                    .elaborate_assume_stmt(prop)
                    .expect("handle contradiction");
                let hypothesis_chunk = hypothesis_constructor.obtain_hypothesis_chunk_within_stmt(
                    stmt,
                    |hypothesis_constructor| {
                        self.inner.transcribe_explicit_hypothesis(
                            hypothesis,
                            prop,
                            hypothesis_constructor,
                        )
                    },
                );
                hypothesis_constructor
                    .stmt_arena_mut()
                    .update(stmt, |entry| {
                        let VdMirStmtData::Assume {
                            hypothesis_chunk_place,
                            ..
                        } = entry.data_mut()
                        else {
                            unreachable!()
                        };
                        hypothesis_chunk_place.set(Ok(hypothesis_chunk));
                    });
            }
            VdMirStmtData::LetAssigned { .. } => {
                let elaboration = self
                    .inner
                    .elaborate_let_assigned_stmt()
                    .expect("handle contradiction");
                todo!();
            }
            VdMirStmtData::Goal { .. } => {
                self.inner
                    .elaborate_goal_stmt()
                    .expect("handle contradiction");
            }
            VdMirStmtData::Have { prop, hint, .. } => {
                self.elaborate_expr(prop, hypothesis_constructor);
                let hypothesis = self
                    .inner
                    .elaborate_have_stmt(stmt, prop, hint, hypothesis_constructor.region_data())
                    .expect("handle contradiction");
                let hypothesis_chunk = hypothesis_constructor.obtain_hypothesis_chunk_within_stmt(
                    stmt,
                    |hypothesis_constructor| {
                        self.inner.transcribe_explicit_hypothesis(
                            hypothesis,
                            prop,
                            hypothesis_constructor,
                        )
                    },
                );
                hypothesis_constructor
                    .stmt_arena_mut()
                    .update(stmt, |entry| {
                        let VdMirStmtData::Have {
                            hypothesis_chunk_place,
                            ..
                        } = entry.data_mut()
                        else {
                            unreachable!()
                        };
                        hypothesis_chunk_place.set(Ok(hypothesis_chunk));
                    });
            }
            VdMirStmtData::Show { .. } => {
                let elaboration = self
                    .inner
                    .elaborate_show_stmt()
                    .expect("handle contradiction");
                todo!();
            }
            VdMirStmtData::Qed {
                goal_and_hypothesis_chunk_place: goal_and_hypothesis_place,
            } => {
                if let Some((goal, _)) = goal_and_hypothesis_place {
                    let hypothesis = self
                        .inner
                        .elaborate_qed_stmt()
                        .expect("handle contradiction");
                    let hypothesis_chunk = hypothesis_constructor
                        .obtain_hypothesis_chunk_within_stmt(stmt, |hypothesis_constructor| {
                            self.inner.transcribe_explicit_hypothesis(
                                hypothesis,
                                goal,
                                hypothesis_constructor,
                            )
                        });
                    hypothesis_constructor
                        .stmt_arena_mut()
                        .update(stmt, |entry| {
                            let VdMirStmtData::Qed {
                                goal_and_hypothesis_chunk_place: Some((_, hypothesis_chunk_place)),
                                ..
                            } = entry.data_mut()
                            else {
                                unreachable!()
                            };
                            hypothesis_chunk_place.set(Ok(hypothesis_chunk));
                        });
                }
            }
        }
    }

    fn elaborate_expr(
        &mut self,
        expr: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        // ad hoc
        // TODO: store expr elaboration in expr arena
        match *hypothesis_constructor.expr_arena()[expr].data() {
            VdMirExprData::Literal(_) | VdMirExprData::Variable(_) => (),
            VdMirExprData::Application {
                function,
                arguments,
            } => {
                if let Some(function) = function.expr() {
                    self.elaborate_expr(function, hypothesis_constructor);
                }
                for arg in arguments {
                    self.elaborate_expr(arg, hypothesis_constructor);
                }
                self.elaborate_application_expr(expr, function, arguments, hypothesis_constructor);
            }
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                // need to do this to avoid rustc complaining
                // we could also unsafe this
                let followers: SmallVec<[(VdMirFunc, VdMirExprIdx); 4]> = followers.to_smallvec();
                let followers: &[(VdMirFunc, VdMirExprIdx)] = &followers;
                self.elaborate_expr(leader, hypothesis_constructor);
                for &(_, follower) in followers {
                    self.elaborate_expr(follower, hypothesis_constructor);
                }
                self.inner
                    .elaborate_folding_separated_list_expr(leader, followers)
            }
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_signature,
            } => {
                // need to do this to avoid rustc complaining
                // we could also unsafe this
                let followers: SmallVec<[(VdMirFunc, VdMirExprIdx); 4]> = followers.to_smallvec();
                let followers: &[(VdMirFunc, VdMirExprIdx)] = &followers;
                self.elaborate_expr(leader, hypothesis_constructor);
                for &(_, follower) in followers {
                    self.elaborate_expr(follower, hypothesis_constructor);
                }
                self.inner.elaborate_chaining_separated_list_expr(
                    leader,
                    followers,
                    joined_signature,
                )
            }
            VdMirExprData::ItemPath(vd_item_path) => (),
        }
        self.inner
            .cache_expr(expr, hypothesis_constructor.region_data());
    }

    fn elaborate_application_expr(
        &mut self,
        expr: VdMirExprIdx,
        function: VdMirFunc,
        arguments: VdMirExprIdxRange,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        match function {
            VdMirFunc::NormalBasePrefixOpr(signature) => match signature.opr {
                VdMirBasePrefixOpr::RingPos => (),
                VdMirBasePrefixOpr::RingNeg => (),
                _ => todo!(),
            },
            VdMirFunc::NormalBaseSeparator(signature) => todo!(),
            VdMirFunc::NormalBaseBinaryOpr(signature) => match signature.opr {
                VdMirBaseBinaryOpr::CommRingSub => (),
                VdMirBaseBinaryOpr::CommFieldDiv => {
                    let _ = self.inner.elaborate_field_div_expr(
                        arguments.last().unwrap(),
                        hypothesis_constructor,
                    );
                    // ad hoc, should save this somewhere
                    // todo!()
                }
            },
            // ad hoc, should check very carefully that one of the following holds:
            // - the base is positive
            // - the base is zero and the exponent is positive
            // - the exponent is a positive integer
            // - the base is nonzero and the exponent is zero
            VdMirFunc::Power(signature) => (),
            VdMirFunc::InSet => todo!(),
            VdMirFunc::NormalBaseSqrt(signature) => (), // ad hoc, should be merged with power
        }
    }
}
