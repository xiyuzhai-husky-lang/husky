use super::*;
use husky_declarative_signature::{
    ExplicitParameterDeclarativeSignatureTemplate, ExplicitParameterDeclarativeSignatureTemplates,
};
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExplicitParameterEtherealSignatureTemplate {
    kind: ExplicitParameterKindTemplate,
    contract: Contract,
    ty: EtherealTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ExplicitParameterKindTemplate {
    Regular,
    Keyed { ident: Ident },
}

impl ExplicitParameterEtherealSignatureTemplate {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature: &ExplicitParameterDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self {
            kind: ExplicitParameterKindTemplate::Regular,
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

    pub fn kind(&self) -> ExplicitParameterKindTemplate {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExplicitParameterEtherealSignatureTemplates {
    data: SmallVec<[ExplicitParameterEtherealSignatureTemplate; 4]>,
}

impl ExplicitParameterEtherealSignatureTemplates {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signatures: &ExplicitParameterDeclarativeSignatureTemplates,
    ) -> EtherealSignatureResult<Self> {
        Ok(ExplicitParameterEtherealSignatureTemplates {
            data: declarative_signatures
                .iter()
                .map(|declarative_signature| {
                    ExplicitParameterEtherealSignatureTemplate::from_declarative(
                        db,
                        declarative_signature,
                    )
                })
                .collect::<EtherealSignatureResult<_>>()?,
        })
    }
}

impl std::ops::Deref for ExplicitParameterEtherealSignatureTemplates {
    type Target = [ExplicitParameterEtherealSignatureTemplate];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
