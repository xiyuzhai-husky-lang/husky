mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;
use husky_declarative_signature::{
    DeclarativeParenicParameters, SpecificDeclarativeParameter,
    SpecificRegularDeclarativeParameterTemplate,
};
use husky_term_prelude::Contract;

// todo: use variable for dependent type
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum SpecificEtherealParameter {
    Regular(RegularSpecificParameter),
    Variadic(SpecificVariadicParameterEtherealSignatureTemplate),
    Keyed(SpecificKeyedParameterEtherealSignatureTemplate),
}

impl SpecificEtherealParameter {
    pub fn from_declarative(
        db: &dyn EtherealSignatureDb,
        param: SpecificDeclarativeParameter,
    ) -> EtherealSignatureResult<Self> {
        Ok(match param {
            SpecificDeclarativeParameter::Regular(declarative_signature_template) => {
                RegularSpecificParameter::from_declarative(db, declarative_signature_template)?
                    .into()
            }
            SpecificDeclarativeParameter::Variadic(_) => todo!(),
            SpecificDeclarativeParameter::Keyed(_) => todo!(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SpecificParameterEtherealSignatureTemplates {
    data: SmallVec<[SpecificEtherealParameter; 4]>,
}

impl SpecificParameterEtherealSignatureTemplates {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature_templates: &DeclarativeParenicParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(SpecificParameterEtherealSignatureTemplates {
            data: declarative_signature_templates
                .iter()
                .copied()
                .map(|declarative_signature_template| {
                    SpecificEtherealParameter::from_declarative(db, declarative_signature_template)
                })
                .collect::<EtherealSignatureResult<_>>()?,
        })
    }
}

impl std::ops::Deref for SpecificParameterEtherealSignatureTemplates {
    type Target = [SpecificEtherealParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl EtherealTermInstantiateRef for SpecificEtherealParameter {
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
