use super::*;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSemSeparatedListDispatch {
    IntAdd,
    Eq,
}

impl<'db> VdSemExprBuilder<'db> {
    pub(super) fn build_separated_list(
        &mut self,
        separator: VdSeparator,
        fragments: &[Either<VdSynExprIdx, VdSynSeparator>],
    ) -> VdSemExprData {
        VdSemExprData::SeparatedList {
            separator,
            // TODO: ad hoc, what to do for separators?
            items: fragments
                .iter()
                .copied()
                .filter_map(|fragment| match fragment {
                    Left(expr) => Some(expr),
                    Right(_) => None,
                })
                .to_vd_sem(self),
            // TODO: ad hoc, should consider much more based on type information, especially space.
            dispatch: match separator {
                VdSeparator::Base(separator) => match separator {
                    VdBaseSeparator::Space => todo!(),
                    VdBaseSeparator::Comma => todo!(),
                    VdBaseSeparator::Semicolon => todo!(),
                    VdBaseSeparator::Add => VdSemSeparatedListDispatch::IntAdd,
                    VdBaseSeparator::Mul => todo!(),
                    VdBaseSeparator::Dot => todo!(),
                    VdBaseSeparator::Eq => VdSemSeparatedListDispatch::Eq,
                },
                VdSeparator::Composite(vd_composite_separator) => todo!(),
            },
        }
    }
}
