use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
pub struct ImplicitParameterEtherealSignatures {
    data: SmallVec<[ImplicitParameterEtherealSignature; 2]>,
}

impl ImplicitParameterEtherealSignatures {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        implicit_paramters: &ImplicitParameterDeclarativeSignatures,
    ) -> EtherealSignatureResult<ImplicitParameterEtherealSignatures> {
        Ok(ImplicitParameterEtherealSignatures {
            data: implicit_paramters
                .data()
                .iter()
                .map(|implicit_parameter| {
                    ImplicitParameterEtherealSignature::from_declarative(db, implicit_parameter)
                })
                .collect::<EtherealSignatureResult<_>>()?,
        })
    }

    pub fn data(&self) -> &[ImplicitParameterEtherealSignature] {
        &self.data
    }
}

impl std::ops::Deref for ImplicitParameterEtherealSignatures {
    type Target = [ImplicitParameterEtherealSignature];

    fn deref(&self) -> &Self::Target {
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
        db: &dyn EtherealSignatureDb,
        implicit_paramter: &ImplicitParameterDeclarativeSignature,
    ) -> EtherealSignatureResult<ImplicitParameterEtherealSignature> {
        todo!()
    }
}
