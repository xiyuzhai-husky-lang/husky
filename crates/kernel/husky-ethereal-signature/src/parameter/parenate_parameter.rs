use super::*;
use husky_declarative_signature::DeclarativeParenateParameters;
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
pub struct EtherealTermParenateParameters {
    data: SmallVec<[EtherealTermRitchieParameter; 4]>,
}

impl EtherealTermParenateParameters {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        params: &DeclarativeParenateParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(EtherealTermParenateParameters {
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

impl std::ops::Deref for EtherealTermParenateParameters {
    type Target = [EtherealTermRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
