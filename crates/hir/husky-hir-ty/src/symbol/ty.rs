use husky_term_prelude::Variance;

use super::*;

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
    pub(crate) fn from_ethereal(symbol: EtherealTermSymbol, db: &::salsa::Db) -> Option<Self> {
        Some(match symbol.index(db).inner() {
            EtherealTermSymbolIndexInner::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::Type {
                attrs,
                variance,
                disambiguator,
            } => HirTypeSymbol::Type {
                attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            },
            EtherealTermSymbolIndexInner::Prop { disambiguator } => {
                todo!()
            }
            EtherealTermSymbolIndexInner::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => todo!(),
            EtherealTermSymbolIndexInner::ConstOther {
                attrs,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemPathLeading {
                disambiguator,
                ty_path,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemOther { disambiguator } => {
                todo!()
            }
            EtherealTermSymbolIndexInner::SelfType => HirTypeSymbol::SelfType,
            EtherealTermSymbolIndexInner::SelfValue => todo!(),
            EtherealTermSymbolIndexInner::SelfLifetime => todo!(),
            EtherealTermSymbolIndexInner::SelfPlace => todo!(),
        })
    }
}
