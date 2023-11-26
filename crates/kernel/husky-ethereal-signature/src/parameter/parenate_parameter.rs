use super::*;
use husky_declarative_signature::DeclarativeParenateParameters;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EtherealTermParenateParameters {
    data: SmallVec<[EtherealRitchieParameter; 4]>,
}

impl EtherealTermParenateParameters {
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        params: &DeclarativeParenateParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(EtherealTermParenateParameters {
            data: params
                .iter()
                .copied()
                .map(|param| EtherealRitchieParameter::from_declarative(db, param))
                .collect::<EtherealTermResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[EtherealRitchieParameter] {
        &self.data
    }
}

impl std::ops::Deref for EtherealTermParenateParameters {
    type Target = [EtherealRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
