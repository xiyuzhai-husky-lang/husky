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
    pub(crate) fn from_ethereal(symbol: EtherealTermSymbol, db: &::salsa::Db) -> Option<Self> {
        Some(match symbol.index(db).inner() {
            EtherealTermSymbolIndexInner::ExplicitLifetime {
                attrs: _,
                variance: _,
                disambiguator: _,
            } => todo!(),
            EtherealTermSymbolIndexInner::ExplicitPlace {
                attrs: _,
                variance: _,
                disambiguator: _,
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
            EtherealTermSymbolIndexInner::Prop { disambiguator: _ } => {
                todo!()
            }
            EtherealTermSymbolIndexInner::ConstPathLeading {
                attrs: _,
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EtherealTermSymbolIndexInner::ConstOther {
                attrs: _,
                disambiguator: _,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemPathLeading {
                disambiguator: _,
                ty_path: _,
            } => todo!(),
            EtherealTermSymbolIndexInner::EphemOther { disambiguator: _ } => {
                todo!()
            }
            EtherealTermSymbolIndexInner::SelfType => HirTypeSymbol::SelfType,
            EtherealTermSymbolIndexInner::SelfValue => todo!(),
            EtherealTermSymbolIndexInner::SelfLifetime => todo!(),
            EtherealTermSymbolIndexInner::SelfPlace => todo!(),
        })
    }
}
