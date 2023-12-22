use super::*;
use husky_declarative_signature::DeclarativeParenateParameters;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EtherealParenateParameters {
    data: SmallVec<[EtherealRitchieParameter; 4]>,
}

impl EtherealParenateParameters {
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        params: &DeclarativeParenateParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(EtherealParenateParameters {
            data: params
                .iter()
                .copied()
                .map(|param| EtherealRitchieParameter::from_declarative(param, db))
                .collect::<EtherealTermResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[EtherealRitchieParameter] {
        &self.data
    }
}

impl std::ops::Deref for EtherealParenateParameters {
    type Target = [EtherealRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
