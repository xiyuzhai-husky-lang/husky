use husky_term_prelude::Variance;

use super::*;

#[salsa::debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HirTypeSymbol {
    Type {
        attrs: HirTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    SelfType,
    SelfLifetime,
    SelfPlace,
}

impl HirTypeSymbol {
    pub(crate) fn from_ethereal(symbol: SymbolEtherealTerm, db: &::salsa::Db) -> Option<Self> {
        Some(match symbol.index(db).inner() {
            EtherealTermSymbolIndexImpl::ExplicitLifetime {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EtherealTermSymbolIndexImpl::ExplicitPlace {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EtherealTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => HirTypeSymbol::Type {
                attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            },
            EtherealTermSymbolIndexImpl::Prop { disambiguator: _ } => {
                todo!()
            }
            EtherealTermSymbolIndexImpl::ConstPathLeading {
                attrs: _,
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EtherealTermSymbolIndexImpl::ConstOther {
                attrs: _,
                disambiguator: _,
            } => todo!(),
            EtherealTermSymbolIndexImpl::EphemPathLeading {
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EtherealTermSymbolIndexImpl::EphemOther { disambiguator: _ } => {
                todo!()
            }
            EtherealTermSymbolIndexImpl::SelfType => HirTypeSymbol::SelfType,
            EtherealTermSymbolIndexImpl::SelfValue => todo!(),
            EtherealTermSymbolIndexImpl::SelfLifetime => todo!(),
            EtherealTermSymbolIndexImpl::SelfPlace => todo!(),
        })
    }
}
