mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use husky_declarative_signature::{
    ExplicitParameterDeclarativeSignatureTemplate, ExplicitParameterDeclarativeSignatureTemplates,
    ExplicitRegularParameterDeclarativeSignatureTemplate,
};
use husky_term_prelude::Contract;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ExplicitParameterEtherealSignatureTemplate {
    Regular(ExplicitRegularParameterEtherealSignatureTemplate),
    Variadic(ExplicitVariadicParameterEtherealSignatureTemplate),
    Keyed(ExplicitKeyedParameterEtherealSignatureTemplate),
}

impl ExplicitParameterEtherealSignatureTemplate {
    pub fn from_declarative_signature(
        db: &dyn EtherealSignatureDb,
        declarative_signature: &ExplicitParameterDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(match declarative_signature {
            ExplicitParameterDeclarativeSignatureTemplate::Regular(declarative_signature_template) => {
                ExplicitRegularParameterEtherealSignatureTemplate::from_declarative_signature_template(
                    db,
                    declarative_signature_template,
                )?
                .into()
            }
            ExplicitParameterDeclarativeSignatureTemplate::Variadic(_) => todo!(),
            ExplicitParameterDeclarativeSignatureTemplate::Keyed(_) => todo!(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ExplicitParameterEtherealSignatureTemplates {
    data: SmallVec<[ExplicitParameterEtherealSignatureTemplate; 4]>,
}

impl ExplicitParameterEtherealSignatureTemplates {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_templates: &ExplicitParameterDeclarativeSignatureTemplates,
    ) -> EtherealSignatureResult<Self> {
        Ok(ExplicitParameterEtherealSignatureTemplates {
            data: declarative_signature_templates
                .iter()
                .map(|declarative_signature_template| {
                    ExplicitParameterEtherealSignatureTemplate::from_declarative_signature(
                        db,
                        declarative_signature_template,
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

impl EtherealTermInstantiateRef for ExplicitParameterEtherealSignatureTemplate {
    type Target = Option<Self>;

    fn instantiate(
        &self,
        db: &dyn EtherealTermDb,
        instantiation: &EtherealTermInstantiation,
    ) -> Self::Target {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ExplicitParameterEtherealSignature {
    Regular(ExplicitRegularParameterEtherealSignature),
    Keyed(ExplicitKeyedParameterEtherealSignature),
    Variadic(ExplicitVariadicParameterEtherealSignature),
}
