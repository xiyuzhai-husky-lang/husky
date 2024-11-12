use super::*;
use smallvec::{smallvec, SmallVec};
use visored_syn_expr::expr::VdSynExprIdxRange;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemSeparator {
    Base(LxTokenIdxRange, VdBaseSeparator),
    Composite(VdSemExprIdx, VdSeparatorClass),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSemSeparatedListDispatch {
    NatAdd,
    NatMul,
    Eq,
    In,
}

impl<'db> VdSemExprBuilder<'db> {
    pub(super) fn build_separated_list(
        &mut self,
        separator_class: VdSeparatorClass,
        items: VdSynExprIdxRange,
        separators: &[VdSynSeparator],
    ) -> (VdSemExprData, VdZfcType) {
        // TODO: ad hoc,should consider more based on type information, especially space.
        let dispatch = match separators.len() {
            0 => todo!(),
            1 => match separators[0] {
                VdSynSeparator::Base(_, base_separator) => match base_separator {
                    VdBaseSeparator::Space => VdSemSeparatedListDispatch::NatMul,
                    VdBaseSeparator::Comma => todo!(),
                    VdBaseSeparator::Semicolon => todo!(),
                    VdBaseSeparator::Add => VdSemSeparatedListDispatch::NatAdd,
                    VdBaseSeparator::Mul => VdSemSeparatedListDispatch::NatMul,
                    VdBaseSeparator::Dot => todo!(),
                    VdBaseSeparator::Eq => VdSemSeparatedListDispatch::Eq,
                    VdBaseSeparator::Subset => todo!(),
                    VdBaseSeparator::Supset => todo!(),
                    VdBaseSeparator::Subseteq => todo!(),
                    VdBaseSeparator::Supseteq => todo!(),
                    VdBaseSeparator::Subseteqq => todo!(),
                    VdBaseSeparator::Supseteqq => todo!(),
                    VdBaseSeparator::Subsetneq => todo!(),
                    VdBaseSeparator::Supsetneq => todo!(),
                    VdBaseSeparator::In => VdSemSeparatedListDispatch::In,
                    VdBaseSeparator::Notin => todo!(),
                    VdBaseSeparator::Times => todo!(),
                    VdBaseSeparator::Otimes => todo!(),
                    VdBaseSeparator::Ne => todo!(),
                },
                VdSynSeparator::Composite(arena_idx, vd_separator_class) => todo!(),
            },
            2 => {
                use husky_print_utils::p;
                p!(separators);
                for item in items {
                    p!(self.syn_expr_arena()[item]);
                }
                todo!()
            }
            _ => todo!(),
        };
        let data = VdSemExprData::SeparatedList {
            separator_class,
            // TODO: ad hoc, what to do for separators?
            items: items.to_vd_sem(self),
            // TODO: ad hoc, should consider much more based on type information, especially space.
            dispatch,
        };
        let ty = self.infer_separated_list_ty(dispatch);
        (data, ty)
    }

    fn infer_separated_list_ty(&mut self, dispatch: VdSemSeparatedListDispatch) -> VdZfcType {
        match dispatch {
            VdSemSeparatedListDispatch::NatAdd => self.zfc_ty_menu().natural_number_ty(),
            VdSemSeparatedListDispatch::NatMul => todo!(),
            VdSemSeparatedListDispatch::Eq | VdSemSeparatedListDispatch::In => {
                self.zfc_ty_menu().natural_number_ty()
            }
        }
    }
}
