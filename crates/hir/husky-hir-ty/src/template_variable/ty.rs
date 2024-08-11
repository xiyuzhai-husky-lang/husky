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
    SelfLifetime,
    SelfPlace,
}

impl HirTypeTemplateVariable {
    pub(crate) fn from_eth(symbol: EthSymbolicVariable, db: &::salsa::Db) -> Option<Self> {
        Some(match symbol.index(db).inner() {
            EthTermSymbolIndexImpl::ExplicitLifetime {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EthTermSymbolIndexImpl::ExplicitPlace {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EthTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => HirTypeTemplateVariable::Type {
                attrs: HirTemplateVariableAttrs::from_eth(attrs)?,
                variance,
                disambiguator,
            },
            EthTermSymbolIndexImpl::Prop { disambiguator: _ } => {
                todo!()
            }
            EthTermSymbolIndexImpl::ConstPathLeading {
                attrs: _,
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EthTermSymbolIndexImpl::ConstOther {
                attrs: _,
                disambiguator: _,
            } => todo!(),
            EthTermSymbolIndexImpl::EphemPathLeading {
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EthTermSymbolIndexImpl::EphemOther { disambiguator: _ } => {
                todo!()
            }
            EthTermSymbolIndexImpl::SelfType => HirTypeTemplateVariable::SelfType,
            EthTermSymbolIndexImpl::SelfValue => todo!(),
            EthTermSymbolIndexImpl::SelfLifetime => todo!(),
            EthTermSymbolIndexImpl::SelfPlace => todo!(),
        })
    }
}
