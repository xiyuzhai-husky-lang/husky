/// this module is for instantiation to use
use crate::{instantiation::*, term::symbolic_variable::EthSymbolicVariable, *};
use husky_dec_signature::parameter::DeclarativeTemplateParameter;
use husky_entity_path::path::ItemPath;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
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
    pub fn from_dec(
        db: &::salsa::Db,
        template_parameters: &[DeclarativeTemplateParameter],
    ) -> EthTermResult<EthTemplateParameters> {
        Ok(EthTemplateParameters {
            data: template_parameters
                .iter()
                .map(|template_parameter| EthTemplateParameter::from_dec(db, template_parameter))
                .collect::<EthTermResult<_>>()?,
        })
    }

    #[inline(always)]
    pub fn data(&self) -> &[EthTemplateParameter] {
        &self.data
    }

    /// returns an empty partial instantiation
    pub fn empty_instantiation_builder<'db>(
        &self,
        path: ItemPath,
        is_associated: bool,
        package_signature_data_result: &'db impl IsPackageEthSignatureData,
    ) -> EthInstantiationBuilder {
        EthInstantiationBuilder::new(
            path,
            self.iter().map(|param| param.symbol()),
            is_associated,
            package_signature_data_result,
        )
    }
}

impl std::ops::Deref for EthTemplateParameters {
    type Target = [EthTemplateParameter];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::derive_debug_with_db]
pub struct EthTemplateParameter {
    annotated_variance: Option<Variance>,
    symbol: EthSymbolicVariable,
    traits: Vec<EthTerm>,
}

impl EthTemplateParameter {
    fn from_dec(
        db: &::salsa::Db,
        declarative_generic_paramter: &DeclarativeTemplateParameter,
    ) -> EthTermResult<Self> {
        Ok(Self {
            annotated_variance: declarative_generic_paramter.annotated_variance(),
            symbol: EthSymbolicVariable::from_dec(db, declarative_generic_paramter.symbol())?,
            traits: declarative_generic_paramter
                .traits()
                .iter()
                .map(|_| todo!())
                .collect(),
        })
    }

    pub fn symbol(&self) -> EthSymbolicVariable {
        self.symbol
    }

    pub fn traits(&self) -> &[EthTerm] {
        self.traits.as_ref()
    }
}
