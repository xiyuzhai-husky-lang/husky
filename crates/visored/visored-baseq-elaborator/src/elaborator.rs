use eterned::db::EternerDb;
use floated_sequential::db::FloaterDb;
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
use visored_opr::separator::VdBaseSeparator;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;

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

pub struct VdBsqElaboratorInner<'db, 'sess> {
    session: &'sess VdBsqSession<'db>,
    expr_to_fld_map: VdMirExprMap<VdMirExprFld<'sess>>,
    pub(crate) hypothesis_constructor: VdBsqHypothesisConstructor<'db, 'sess>,
}

pub type VdBsqElaborator<'db, 'sess> = VdMirSequentialElaborator<VdBsqElaboratorInner<'db, 'sess>>;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn new(session: &'sess VdBsqSession<'db>, region_data: VdMirExprRegionDataRef) -> Self {
        Self {
            session,
            hypothesis_constructor: VdBsqHypothesisConstructor::new(session),
            expr_to_fld_map: VdMirExprMap::new2(region_data.expr_arena),
        }
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
            VdMirFunc::NormalBasePrefixOpr(vd_base_prefix_opr_signature) => todo!(),
            VdMirFunc::NormalBaseSeparator(vd_base_separator_signature) => todo!(),
            VdMirFunc::NormalBaseBinaryOpr(vd_base_binary_opr_signature) => todo!(),
            VdMirFunc::Power(vd_power_signature) => (), // ad hoc
            VdMirFunc::InSet => todo!(),
            VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
            VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
        }
    }

    fn elaborate_chaining_separated_list_expr(
        &mut self,
        leader: VdMirExprIdx,
        followers: &[(VdMirFunc, VdMirExprIdx)],
        joined_separator_and_signature: Option<(VdBaseSeparator, VdBaseSeparatorSignature)>,
    ) {
        // todo!()
    }

    fn cache_expr(&mut self, expr: VdMirExprIdx, region_data: VdMirExprRegionDataRef) {
        self.cache_expr_fld(expr, region_data);
    }

    fn prune_explicit_hypothesis(
        &mut self,
        hypothesis: Self::HypothesisIdx,
        goal: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdMirHypothesisIdx {
        let construction = match *self.hypothesis_constructor.arena()[hypothesis].construction() {
            VdBsqHypothesisConstruction::Sorry => VdMirHypothesisConstruction::Sorry,
            VdBsqHypothesisConstruction::Apply {
                path,
                is_real_coercion,
            } => VdMirHypothesisConstruction::Apply {
                path,
                is_real_coercion: self.prune_coercion(is_real_coercion, hypothesis_constructor),
            },
            VdBsqHypothesisConstruction::Phantom(phantom_data) => todo!(),
            VdBsqHypothesisConstruction::Assume => todo!(),
        };
        hypothesis_constructor.construct_new_hypothesis(goal, construction)
    }
}
