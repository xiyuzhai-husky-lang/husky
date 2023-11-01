use crate::*;
use husky_hir_ty::ritchie::HirRitchieParameter;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirParenateParameters {
    data: SmallVec<[HirRitchieParameter; 4]>,
}

impl HirParenateParameters {
    pub(crate) fn from_ethereal(
        params: &EtherealTermParenateParameters,
        db: &dyn HirDeclDb,
    ) -> Self {
        HirParenateParameters {
            data: params
                .iter()
                .copied()
                .map(|param| HirRitchieParameter::from_ethereal(param, db))
                .collect(),
        }
    }
}

impl std::ops::Deref for HirParenateParameters {
    type Target = [HirRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
