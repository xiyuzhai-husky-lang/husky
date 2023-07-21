use super::*;
use husky_declarative_signature::DeclarativeParenicParameters;
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
pub struct ParenicEtherealParameters {
    data: SmallVec<[EtherealTermRitchieParameter; 4]>,
}

impl ParenicEtherealParameters {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        params: &DeclarativeParenicParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(ParenicEtherealParameters {
            data: params
                .iter()
                .copied()
                .map(|param| EtherealTermRitchieParameter::from_declarative(db, param))
                .collect::<EtherealTermResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[EtherealTermRitchieParameter] {
        &self.data
    }
}

impl std::ops::Deref for ParenicEtherealParameters {
    type Target = [EtherealTermRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
