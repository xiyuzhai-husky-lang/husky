/// this module is for instantiation to use
use crate::{instantiation::*, *};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub struct EtherealGenericParameters {
    data: SmallVec<[EtherealGenericParameter; 2]>,
}

impl EtherealGenericParameters {
    pub fn from_declarative(
        db: &dyn EtherealTermDb,
        generic_paramters: &DeclarativeGenericParameters,
    ) -> EtherealTermResult<EtherealGenericParameters> {
        Ok(EtherealGenericParameters {
            data: generic_paramters
                .data()
                .iter()
                .map(|generic_parameter| {
                    EtherealGenericParameter::from_declarative(db, generic_parameter)
                })
                .collect::<EtherealTermResult<_>>()?,
        })
    }

    #[inline(always)]
    pub fn data(&self) -> &[EtherealGenericParameter] {
        &self.data
    }

    pub fn instantiation(
        &self,
        self_ty_parameter: Option<EtherealTermSymbol>,
    ) -> EtherealTermPartialInstantiation {
        unsafe {
            EtherealTermPartialInstantiation::new(
                self.iter()
                    .map(|param| param.symbol())
                    .chain(self_ty_parameter),
            )
        }
    }
}

impl std::ops::Deref for EtherealGenericParameters {
    type Target = [EtherealGenericParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct EtherealGenericParameter {
    annotated_variance: Option<Variance>,
    symbol: EtherealTermSymbol,
    traits: Vec<EtherealTerm>,
}

impl EtherealGenericParameter {
    fn from_declarative(
        db: &dyn EtherealTermDb,
        declarative_generic_paramter: &DeclarativeGenericParameter,
    ) -> EtherealTermResult<Self> {
        Ok(Self {
            annotated_variance: declarative_generic_paramter.annotated_variance(),
            symbol: EtherealTermSymbol::from_declarative(
                db,
                declarative_generic_paramter.symbol(),
            )?,
            traits: declarative_generic_paramter
                .traits()
                .iter()
                .map(|_| todo!())
                .collect(),
        })
    }

    pub fn symbol(&self) -> EtherealTermSymbol {
        self.symbol
    }

    pub fn traits(&self) -> &[EtherealTerm] {
        self.traits.as_ref()
    }
}
