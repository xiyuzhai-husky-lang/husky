mod r#const;
mod lifetime;
mod place;
mod ty;

pub use self::lifetime::*;
pub use self::place::*;
pub use self::r#const::*;
pub use self::ty::*;

use crate::*;
use husky_ethereal_term::{
    EtherealTemplateSymbolAttrs, EtherealTermSymbol, EtherealTermSymbolIndexInner,
};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateSymbol {
    Type(HirTypeSymbol),
    Const(HirConstSymbol),
    Lifetime(HirLifetimeSymbol),
    Place(HirPlaceSymbol),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirSymbolAttrs();

impl HirSymbolAttrs {
    pub(crate) fn from_ethereal(attrs: EtherealTemplateSymbolAttrs) -> Option<Self> {
        (!attrs.phantom()).then_some(Self())
    }
}

impl HirTemplateSymbol {
    pub fn from_ethereal(symbol: EtherealTermSymbol, db: &dyn HirTypeDb) -> Option<Self> {
        hir_template_symbol_from_ethereal(db, symbol)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_template_symbol_from_ethereal(
    db: &dyn HirTypeDb,
    symbol: EtherealTermSymbol,
) -> Option<HirTemplateSymbol> {
    match symbol.index(db).inner() {
        EtherealTermSymbolIndexInner::ExplicitLifetime {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirLifetimeSymbol {
                attrs: HirSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EtherealTermSymbolIndexInner::ExplicitPlace {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirPlaceSymbol {
                attrs: HirSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EtherealTermSymbolIndexInner::Type {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirTypeSymbol::Type {
                attrs: HirSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EtherealTermSymbolIndexInner::Prop { disambiguator } => todo!(),
        EtherealTermSymbolIndexInner::ConstPathLeading {
            attrs,
            disambiguator,
            ty_path,
        } => Some(
            HirConstSymbol::new(
                db,
                HirType::from_ethereal(symbol.ty(db), db),
                HirConstSymbolIndex::PathLeading {
                    disambiguator,
                    ty_path,
                },
            )
            .into(),
        ),
        EtherealTermSymbolIndexInner::ConstOther {
            attrs,
            disambiguator,
        } => Some(
            HirConstSymbol::new(
                db,
                HirType::from_ethereal(symbol.ty(db), db),
                HirConstSymbolIndex::Other { disambiguator },
            )
            .into(),
        ),
        EtherealTermSymbolIndexInner::EphemPathLeading {
            disambiguator,
            ty_path,
        } => todo!(),
        EtherealTermSymbolIndexInner::EphemOther { disambiguator } => {
            todo!()
        }
        EtherealTermSymbolIndexInner::SelfType => Some(HirTypeSymbol::SelfType.into()),
        EtherealTermSymbolIndexInner::SelfValue => todo!(),
        EtherealTermSymbolIndexInner::SelfLifetime => Some(HirTypeSymbol::SelfLifetime.into()),
        EtherealTermSymbolIndexInner::SelfPlace => Some(HirTypeSymbol::SelfPlace.into()),
    }
}
