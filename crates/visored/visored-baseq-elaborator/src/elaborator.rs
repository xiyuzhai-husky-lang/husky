use crate::{
    call::stack::VdBsqCallStack,
    expr::{VdMirExprFld, VdMirExprFldData},
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
use smallvec::*;
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
    pattern::VdMirPattern,
    region::VdMirExprRegionDataRef,
    stmt::{block::VdMirBlockKind, VdMirStmtData, VdMirStmtIdx},
};
use visored_mir_opr::{opr::binary::VdMirBaseBinaryOpr, separator::VdMirBaseSeparator};
use visored_signature::{
    menu::{vd_signature_menu, VdSignatureMenu},
    signature::separator::base::VdBaseSeparatorSignature,
};
use visored_term::{
    menu::{vd_ty_menu, VdTypeMenu},
    term::menu::{vd_term_menu, VdTermMenu},
};

pub struct VdBsqElaboratorInner<'db, 'sess> {
    session: &'sess VdBsqSession<'db>,
    term_menu: &'db VdTermMenu,
    ty_menu: &'db VdTypeMenu,
    signature_menu: &'db VdSignatureMenu,
    expr_to_fld_map: VdMirExprMap<VdMirExprFld<'sess>>,
    miracle: Miracle,
    pub(crate) hypothesis_constructor: VdBsqHypothesisConstructor<'db, 'sess>,
    pub(crate) call_stack: VdBsqCallStack,
}

pub type VdBsqElaborator<'db, 'sess> = VdMirSequentialElaborator<VdBsqElaboratorInner<'db, 'sess>>;

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn new(session: &'sess VdBsqSession<'db>, region_data: VdMirExprRegionDataRef) -> Self {
        Self {
            session,
            term_menu: vd_term_menu(session.eterner_db()),
            ty_menu: vd_ty_menu(session.eterner_db()),
            signature_menu: vd_signature_menu(session.eterner_db()),
            hypothesis_constructor: VdBsqHypothesisConstructor::new(session),
            expr_to_fld_map: VdMirExprMap::new2(region_data.expr_arena),
            miracle: Miracle::new_uninitialized(),
            call_stack: VdBsqCallStack::new(),
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

    pub fn term_menu(&self) -> &'db VdTermMenu {
        self.term_menu
    }

    pub fn ty_menu(&self) -> &'db VdTypeMenu {
        self.ty_menu
    }

    pub fn signature_menu(&self) -> &'db VdSignatureMenu {
        self.signature_menu
    }

    pub fn call_stack(&self) -> &VdBsqCallStack {
        &self.call_stack
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

    fn enter_block(&mut self, kind: VdMirBlockKind) {
        match kind {
            VdMirBlockKind::Paragraph | VdMirBlockKind::Sentence => (),
            VdMirBlockKind::Environment | VdMirBlockKind::Division => {
                self.hypothesis_constructor.enter_block()
            }
        }
    }

    fn exit_block(&mut self, kind: VdMirBlockKind) {
        match kind {
            VdMirBlockKind::Paragraph | VdMirBlockKind::Sentence => (),
            VdMirBlockKind::Environment | VdMirBlockKind::Division => {
                self.hypothesis_constructor.exit_block()
            }
        }
    }

    fn elaborate_let_assigned_stmt(
        &mut self,
        pattern: &VdMirPattern,
        assignment: VdMirExprIdx,
        region_data: VdMirExprRegionDataRef,
    ) -> VdBsqHypothesisResult<'sess, ()> {
        match *pattern {
            VdMirPattern::Letter {
                letter,
                symbol_local_defn,
            } => {
                let assignment = self.expr_fld(assignment);
                let variable = self.mk_expr(
                    VdMirExprFldData::Variable(letter, symbol_local_defn),
                    assignment.ty(),
                );
                let signature = self.eq_signature(assignment.ty());
                let eq_expr_data = VdMirExprFldData::ChainingSeparatedList {
                    leader: variable,
                    followers: smallvec![(
                        VdMirFunc::NormalBaseSeparator(signature),
                        self.mk_zero()
                    )],
                    joined_signature: None,
                };
                let prop = self.mk_expr(eq_expr_data, self.ty_menu().prop);
                self.obvious(prop).map(|_| ())
            }
        }
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

    fn elaborate_field_div_expr(
        &mut self,
        divisor: VdMirExprIdx,
        hypothesis_constructor: &mut VdMirHypothesisConstructor,
    ) -> VdBsqHypothesisResult<'sess, VdBsqHypothesisIdx<'sess>> {
        let divisor = self.expr_fld(divisor);
        let signature = if divisor.ty() == self.ty_menu().nat {
            self.signature_menu().nat_ne
        } else {
            todo!()
        };
        let prop = self.mk_expr(
            VdMirExprFldData::ChainingSeparatedList {
                leader: divisor,
                followers: smallvec![(VdMirFunc::NormalBaseSeparator(signature), self.mk_zero())],
                joined_signature: None,
            },
            self.ty_menu().prop,
        );
        self.obvious(prop)
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
            VdBsqHypothesisConstruction::ExprEquivalent { hypothesis } => {
                VdMirHypothesisConstruction::ExprEquivalent {}
            }
            VdBsqHypothesisConstruction::CommRing => VdMirHypothesisConstruction::CommRing,
        };
        hypothesis_constructor.construct_new_hypothesis(goal, construction)
    }
}
