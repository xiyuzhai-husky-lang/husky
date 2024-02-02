/// this module is for instantiation to use
use crate::{instantiation::*, *};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db]
pub struct EthTemplateParameters {
    data: SmallVec<[EthTemplateParameter; 2]>,
}

impl<'a> IntoIterator for &'a EthTemplateParameters {
    type Item = &'a EthTemplateParameter;

    type IntoIter = impl Iterator<Item = &'a EthTemplateParameter> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl EthTemplateParameters {
    pub fn from_declarative(
        db: &::salsa::Db,
        template_parameters: &[DeclarativeTemplateParameter],
    ) -> EthTermResult<EthTemplateParameters> {
        Ok(EthTemplateParameters {
            data: template_parameters
                .iter()
                .map(|template_parameter| {
                    EthTemplateParameter::from_declarative(db, template_parameter)
                })
                .collect::<EthTermResult<_>>()?,
        })
    }

    #[inline(always)]
    pub fn data(&self) -> &[EthTemplateParameter] {
        &self.data
    }

    /// returns an empty partial instantiation
    pub fn empty_instantiation_builder(&self, is_associated: bool) -> EtherealInstantiationBuilder {
        EtherealInstantiationBuilder::new(self.iter().map(|param| param.symbol()), is_associated)
    }
}

impl std::ops::Deref for EthTemplateParameters {
    type Target = [EthTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db]
pub struct EthTemplateParameter {
    annotated_variance: Option<Variance>,
    symbol: EthSymbol,
    traits: Vec<EthTerm>,
}

impl EthTemplateParameter {
    fn from_declarative(
        db: &::salsa::Db,
        declarative_generic_paramter: &DeclarativeTemplateParameter,
    ) -> EthTermResult<Self> {
        Ok(Self {
            annotated_variance: declarative_generic_paramter.annotated_variance(),
            symbol: EthSymbol::from_declarative(db, declarative_generic_paramter.symbol())?,
            traits: declarative_generic_paramter
                .traits()
                .iter()
                .map(|_| todo!())
                .collect(),
        })
    }

    pub fn symbol(&self) -> EthSymbol {
        self.symbol
    }

    pub fn traits(&self) -> &[EthTerm] {
        self.traits.as_ref()
    }
}
