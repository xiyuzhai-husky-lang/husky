use super::*;
use husky_declarative_signature::{
    ExplicitParameterDeclarativeSignature, ExplicitParameterDeclarativeSignatureTemplates,
};
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitParameterEtherealSignature {
    kind: ExplicitParameterKind,
    contract: Contract,
    ty: EtherealTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ExplicitParameterKind {
    Regular,
    Keyed { ident: Ident },
}

impl ExplicitParameterEtherealSignature {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature: &ExplicitParameterDeclarativeSignature,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            kind: ExplicitParameterKind::Regular,
            contract: declarative_signature.contract(),
            ty: EtherealTerm::ty_from_declarative(db, declarative_signature.ty())?,
        })
    }

    pub fn contract(&self) -> Contract {
        self.contract
    }

    pub fn ty(&self) -> EtherealTerm {
        self.ty
    }

    pub fn kind(&self) -> ExplicitParameterKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExplicitParameterEtherealSignatures {
    data: SmallVec<[ExplicitParameterEtherealSignature; 4]>,
}

impl ExplicitParameterEtherealSignatures {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signatures: &ExplicitParameterDeclarativeSignatureTemplates,
    ) -> EtherealSignatureResult<Self> {
        Ok(ExplicitParameterEtherealSignatures {
            data: declarative_signatures
                .iter()
                .map(|declarative_signature| {
                    ExplicitParameterEtherealSignature::from_declarative(db, declarative_signature)
                })
                .collect::<EtherealSignatureResult<_>>()?,
        })
    }
}

impl std::ops::Deref for ExplicitParameterEtherealSignatures {
    type Target = [ExplicitParameterEtherealSignature];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
