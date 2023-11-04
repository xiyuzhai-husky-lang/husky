use crate::*;
use husky_hir_ty::ritchie::HirRitchieParameter;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirRitchieParameters {
    data: SmallVec<[HirRitchieParameter; 4]>,
}

impl HirRitchieParameters {
    pub(crate) fn from_ethereal(
        params: &EtherealTermParenateParameters,
        db: &dyn HirDeclDb,
    ) -> Self {
        HirRitchieParameters {
            data: params
                .iter()
                .copied()
                .map(|param| HirRitchieParameter::from_ethereal(param, db))
                .collect(),
        }
    }
}

impl std::ops::Deref for HirRitchieParameters {
    type Target = [HirRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
