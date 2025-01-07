use crate::{
    expr::VdMirExprFld,
    hypothesis::{
        construction::VdBsqHypothesisConstruction,
        constructor::VdBsqHypothesisConstructor,
        contradiction::{VdBsqHypothesisContradiction, VdBsqHypothesisResult},
        VdBsqHypothesisIdx,
    },
    session::VdBsqSession,
};
use eterned::db::EternerDb;
use floated_sequential::db::FloaterDb;
use miracle::{HasMiracle, Miracle};
use std::marker::PhantomData;
use visored_mir_expr::{
    elaborator::linear::{IsVdMirSequentialElaboratorInner, VdMirSequentialElaborator},
    expr::{
        application::VdMirFunc, VdMirExprArenaRef, VdMirExprIdx, VdMirExprIdxRange, VdMirExprMap,
        VdMirExprOrderedMap,
    },
    hint::VdMirHintIdx,
    hypothesis::{
        construction::VdMirHypothesisConstruction, constructor::VdMirHypothesisConstructor,
        VdMirHypothesisIdx,
    },
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtData, VdMirStmtIdx},
};
use visored_mir_opr::{opr::binary::VdMirBaseBinaryOpr, separator::VdMirBaseSeparator};
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

pub struct VdBsqElaboratorInner<'db, 'sess> {
    session: &'sess VdBsqSession<'db>,
    expr_to_fld_map: VdMirExprMap<VdMirExprFld<'sess>>,
    miracle: Miracle,
    pub(crate) hypothesis_constructor: VdBsqHypothesisConstructor<'db, 'sess>,
}

pub type VdBsqElaborator<'db, 'sess> = VdMirSequentialElaborator<VdBsqElaboratorInner<'db, 'sess>>;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn new(session: &'sess VdBsqSession<'db>, region_data: VdMirExprRegionDataRef) -> Self {
        Self {
            session,
            hypothesis_constructor: VdBsqHypothesisConstructor::new(session),
            expr_to_fld_map: VdMirExprMap::new2(region_data.expr_arena),
            miracle: Miracle::new_uninitialized(),
        }
    }
}

impl<'db, 'sess> HasMiracle for VdBsqElaboratorInner<'db, 'sess> {
    fn miracle(&self) -> &Miracle {
        &self.miracle
    }

    fn miracle_mut(&mut self) -> &mut Miracle {
        &mut self.miracle
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn session(&self) -> &'sess VdBsqSession<'db> {
        self.session
    }

    pub fn eterner_db(&self) -> &'db EternerDb {
        self.session.eterner_db()
    }

    pub fn floater_db(&self) -> &'sess FloaterDb {
        self.session.floater_db()
    }

    #[track_caller]
    pub fn expr_fld(&self, expr: VdMirExprIdx) -> VdMirExprFld<'sess> {
        self.expr_to_fld_map[expr]
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub(crate) fn save_expr_fld(&mut self, expr: VdMirExprIdx, fld: VdMirExprFld<'sess>) {
        self.expr_to_fld_map.insert_new(expr, fld);
    }
}

impl<'db, 'sess> IsVdMirSequentialElaboratorInner for VdBsqElaboratorInner<'db, 'sess> {
    type HypothesisIdx = VdBsqHypothesisIdx<'sess>;
    type Contradiction = VdBsqHypothesisContradiction<'sess>;

    fn elaborate_let_assigned_stmt(&mut self) -> VdBsqHypothesisResult<'sess, ()> {
        Ok(())
    }

    fn elaborate_let_placeholder_stmt(&mut self) -> VdBsqHypothesisResult<'sess, ()> {
        Ok(())
    }

    fn elaborate_assume_stmt(
        &mut self,
        prop: VdMirExprIdx,
    ) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        Ok(self
            .hypothesis_constructor
            .construct_new_hypothesis(self.expr_fld(prop), VdBsqHypothesisConstruction::Assume))
    }

    fn elaborate_goal_stmt(&mut self) -> VdBsqHypothesisResult<'sess, ()> {
        Ok(())
    }

    fn elaborate_have_stmt(
        &mut self,
        stmt: VdMirStmtIdx,
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        region_data: VdMirExprRegionDataRef,
    ) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        let prop = self.expr_to_fld_map[prop];
        match hint {
            Some(hint) => todo!(),
            None => self.obvious(prop),
        }
    }

    fn elaborate_show_stmt(&mut self) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        todo!()
    }

    fn elaborate_qed_stmt(&mut self) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        todo!()
    }

    fn elaborate_application_expr(
        &mut self,
        function: VdMirFunc,
        arguments: VdMirExprIdxRange,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) {
        match function {
            VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
            VdMirFunc::NormalBaseSeparator(signature) => todo!(),
            VdMirFunc::NormalBaseBinaryOpr(signature) => match signature.opr {
                VdMirBaseBinaryOpr::CommRingSub => (),
                VdMirBaseBinaryOpr::CommFieldDiv => todo!(),
            },
            VdMirFunc::Power(signature) => (), // ad hoc
            VdMirFunc::InSet => todo!(),
            VdMirFunc::NormalBaseSqrt(signature) => todo!(),
        }
    }

    fn elaborate_folding_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
    ) {
        let (fst_func, fst) = followers[0];
        let VdMirFunc::NormalBaseSeparator(fst_signature) = fst_func else {
            unreachable!()
        };
        match fst_signature.opr() {
            VdMirBaseSeparator::CommRingAdd => (),
            VdMirBaseSeparator::CommRingMul => (),
            VdMirBaseSeparator::Eq => todo!(),
            VdMirBaseSeparator::Ne => todo!(),
            VdMirBaseSeparator::Lt => todo!(),
            VdMirBaseSeparator::Gt => todo!(),
            VdMirBaseSeparator::Le => todo!(),
            VdMirBaseSeparator::Ge => todo!(),
            VdMirBaseSeparator::Subset => todo!(),
            VdMirBaseSeparator::Supset => todo!(),
            VdMirBaseSeparator::Subseteq => todo!(),
            VdMirBaseSeparator::Supseteq => todo!(),
            VdMirBaseSeparator::Subseteqq => todo!(),
            VdMirBaseSeparator::Supseteqq => todo!(),
            VdMirBaseSeparator::Subsetneq => todo!(),
            VdMirBaseSeparator::Supsetneq => todo!(),
            VdMirBaseSeparator::In => todo!(),
            VdMirBaseSeparator::Notin => todo!(),
            VdMirBaseSeparator::SetTimes => todo!(),
            VdMirBaseSeparator::TensorOtimes => todo!(),
        }
    }

    fn elaborate_chaining_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_signature: Option<VdBaseSeparatorSignature>,
    ) {
        // todo!()
    }

    fn cache_expr(&mut self, expr: VdMirExprIdx, region_data: VdMirExprRegionDataRef) {
        self.cache_expr_fld(expr, region_data);
    }

    fn transcribe_explicit_hypothesis(
        &mut self,
        hypothesis: Self::HypothesisIdx,
        goal: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        let construction = match *self.hypothesis_constructor.arena()[hypothesis].construction() {
            VdBsqHypothesisConstruction::Sorry => VdMirHypothesisConstruction::Sorry,
            VdBsqHypothesisConstruction::TermTrivial(b) => {
                VdMirHypothesisConstruction::TermTrivial(b)
            }
            VdBsqHypothesisConstruction::Apply {
                path,
                is_real_coercion,
            } => VdMirHypothesisConstruction::Apply {
                path,
                is_real_coercion: self
                    .transcribe_coercion(is_real_coercion, hypothesis_constructor),
            },
            VdBsqHypothesisConstruction::Assume => VdMirHypothesisConstruction::Assume,
            VdBsqHypothesisConstruction::TermEquivalent { hypothesis } => {
                VdMirHypothesisConstruction::TermEquivalent {}
            }
            VdBsqHypothesisConstruction::CommRing => VdMirHypothesisConstruction::CommRing,
        };
        hypothesis_constructor.construct_new_hypothesis(goal, construction)
    }
}
