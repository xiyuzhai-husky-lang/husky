use super::*;
use husky_dec_signature::parameter::DeclarativeParenateParameters;
use husky_eth_term::term::ritchie::EtherealRitchieParameter;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub struct EtherealParenateParameters {
    data: SmallVec<[EtherealRitchieParameter; 4]>,
}

impl EtherealParenateParameters {
    pub(crate) fn from_dec(
        db: &::salsa::Db,
        params: &DeclarativeParenateParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(EtherealParenateParameters {
            data: params
                .iter()
                .copied()
                .map(|param| EtherealRitchieParameter::from_dec(param, db))
                .collect::<EthTermResult<_>>()?,
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
