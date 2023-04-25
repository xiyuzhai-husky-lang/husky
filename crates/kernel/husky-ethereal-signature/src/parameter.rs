use crate::*;
use husky_declarative_signature::{
    ImplicitParameterDeclarativeSignature, ImplicitParameterDeclarativeSignatures,
};
use husky_term_prelude::Variance;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
pub struct ImplicitParameterEtherealSignatures {
    data: SmallVec<[ImplicitParameterEtherealSignature; 2]>,
}
impl ImplicitParameterEtherealSignatures {
    pub(crate) fn from_declarative(
        implicit_paramters: &ImplicitParameterDeclarativeSignatures,
    ) -> EtherealSignatureResult<ImplicitParameterEtherealSignatures> {
        Ok(ImplicitParameterEtherealSignatures {
            data: implicit_paramters
                .data()
                .iter()
                .map(ImplicitParameterEtherealSignature::from_declarative)
                .collect::<EtherealSignatureResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[ImplicitParameterEtherealSignature] {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImplicitParameterEtherealSignature {
    annotated_variance: Option<Variance>,
    symbol: EtherealTermSymbol,
    traits: Vec<EtherealTerm>,
}

impl ImplicitParameterEtherealSignature {
    fn from_declarative(
        implicit_paramter: &ImplicitParameterDeclarativeSignature,
    ) -> EtherealSignatureResult<ImplicitParameterEtherealSignature> {
        todo!()
    }
}
