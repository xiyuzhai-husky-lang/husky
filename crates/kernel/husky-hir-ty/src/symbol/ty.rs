use husky_term_prelude::Variance;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirTypeSymbol {
    pub(super) attrs: HirSymbolAttrs,
    pub(super) variance: Option<Variance>,
    pub(super) disambiguator: u8,
}

impl HirTypeSymbol {
    pub(crate) fn from_ethereal(symbol: EtherealTermSymbol, db: &dyn HirTypeDb) -> Self {
        match symbol.index(db).inner() {
            EtherealTermSymbolIndexInner::Lifetime {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
            EtherealTermSymbolIndexInner::Type {
                attrs,
                variance,
                disambiguator,
            } => todo!(),
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
            EtherealTermSymbolIndexInner::SelfType => todo!(),
            EtherealTermSymbolIndexInner::SelfValue => todo!(),
        }
    }
}
