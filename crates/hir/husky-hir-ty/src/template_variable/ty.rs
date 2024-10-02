use husky_term_prelude::Variance;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTypeTemplateVariable {
    Type {
        attrs: HirTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    SelfType,
}

impl HirTypeTemplateVariable {
    pub(crate) fn from_eth(symbol: EthSymbolicVariable, db: &::salsa::Db) -> Option<Self> {
        Some(match symbol.index(db).inner() {
            EthTermVariableIndexImpl::ExplicitLifetime {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EthTermVariableIndexImpl::ExplicitPlace {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EthTermVariableIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => HirTypeTemplateVariable::Type {
                attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                variance,
                disambiguator,
            },
            EthTermVariableIndexImpl::Prop { disambiguator: _ } => {
                todo!()
            }
            EthTermVariableIndexImpl::ConstPathLeading {
                attrs: _,
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EthTermVariableIndexImpl::ConstOther {
                attrs: _,
                disambiguator: _,
            } => todo!(),
            EthTermVariableIndexImpl::EphemPathLeading {
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EthTermVariableIndexImpl::EphemOther { disambiguator: _ } => {
                todo!()
            }
            EthTermVariableIndexImpl::SelfType => HirTypeTemplateVariable::SelfType,
            EthTermVariableIndexImpl::SelfValue => todo!(),
            EthTermVariableIndexImpl::SelfLifetime => todo!(),
            EthTermVariableIndexImpl::SelfPlace => todo!(),
        })
    }
}
