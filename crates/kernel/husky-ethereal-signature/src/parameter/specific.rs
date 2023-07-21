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

// todo: merge this with EtherealTermRitchieParameter
// todo: use variable for dependent type
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum SpecificEtherealParameter {
    Regular(RegularSpecificParameter),
    Variadic(SpecificVariadicParameterEtherealSignatureTemplate),
    Keyed(SpecificKeyedParameterEtherealSignatureTemplate),
}

impl Into<EtherealTermRitchieParameter> for SpecificEtherealParameter {
    fn into(self) -> EtherealTermRitchieParameter {
        match self {
            SpecificEtherealParameter::Regular(_) => todo!(),
            SpecificEtherealParameter::Variadic(_) => todo!(),
            SpecificEtherealParameter::Keyed(_) => todo!(),
        }
    }
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
#[salsa::derive_debug_with_db(db = EtherealSignatureDb)]
pub struct ParenicEtherealParameters {
    data: SmallVec<[EtherealTermRitchieParameter; 4]>,
}

impl ParenicEtherealParameters {
    pub(crate) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        params: &DeclarativeParenicParameters,
    ) -> EtherealSignatureResult<Self> {
        Ok(ParenicEtherealParameters {
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

impl std::ops::Deref for ParenicEtherealParameters {
    type Target = [EtherealTermRitchieParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl EtherealTermInstantiate for SpecificEtherealParameter {
    type Target = Self;

    fn instantiate(
        self,
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
