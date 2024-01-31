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
    EtherealTemplateSymbolAttrs, EtherealTermSymbol, EtherealTermSymbolIndexImpl,
};
use husky_term_prelude::template_symbol_class::TermTemplateSymbolClass;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum HirTemplateSymbol {
    Type(HirTypeSymbol),
    Const(HirConstSymbol),
    Lifetime(HirLifetimeSymbol),
    Place(HirPlaceSymbol),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirTemplateSymbolAttrs {
    class: HirTemplateSymbolClass,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateSymbolClass {
    Comptime,
    Runtime,
}

impl HirTemplateSymbolClass {
    fn from_term(class: TermTemplateSymbolClass) -> Option<Self> {
        match class {
            TermTemplateSymbolClass::Phantom => None,
            TermTemplateSymbolClass::Runtime => Some(HirTemplateSymbolClass::Runtime),
            TermTemplateSymbolClass::Comptime => Some(HirTemplateSymbolClass::Comptime),
        }
    }
}

impl HirTemplateSymbolAttrs {
    pub(crate) fn from_ethereal(attrs: EtherealTemplateSymbolAttrs) -> Option<Self> {
        Some(Self {
            class: HirTemplateSymbolClass::from_term(attrs.class)?,
        })
    }
}

impl HirTemplateSymbol {
    pub fn from_ethereal(symbol: EtherealTermSymbol, db: &::salsa::Db) -> Option<Self> {
        hir_template_symbol_from_ethereal(db, symbol)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_template_symbol_from_ethereal(
    db: &::salsa::Db,
    symbol: EtherealTermSymbol,
) -> Option<HirTemplateSymbol> {
    match symbol.index(db).inner() {
        EtherealTermSymbolIndexImpl::ExplicitLifetime {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirLifetimeSymbol {
                attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EtherealTermSymbolIndexImpl::ExplicitPlace {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirPlaceSymbol {
                attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EtherealTermSymbolIndexImpl::Type {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirTypeSymbol::Type {
                attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                variance,
                disambiguator,
            }
            .into(),
        ),
        EtherealTermSymbolIndexImpl::Prop { disambiguator: _ } => todo!(),
        EtherealTermSymbolIndexImpl::ConstPathLeading {
            attrs,
            disambiguator,
            ty_path,
        } => Some(
            HirConstSymbol::new(
                db,
                HirType::from_ethereal(symbol.ty(db), db)?,
                HirConstSymbolIndex::PathLeading {
                    attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                    disambiguator,
                    ty_path,
                },
            )
            .into(),
        ),
        EtherealTermSymbolIndexImpl::ConstOther {
            attrs,
            disambiguator,
        } => Some(
            HirConstSymbol::new(
                db,
                HirType::from_ethereal(symbol.ty(db), db)?,
                HirConstSymbolIndex::Other {
                    attrs: HirTemplateSymbolAttrs::from_ethereal(attrs)?,
                    disambiguator,
                },
            )
            .into(),
        ),
        EtherealTermSymbolIndexImpl::EphemPathLeading {
            disambiguator: _,
            ty_path: _,
        } => None,
        EtherealTermSymbolIndexImpl::EphemOther { disambiguator: _ } => None,
        EtherealTermSymbolIndexImpl::SelfType => Some(HirTypeSymbol::SelfType.into()),
        EtherealTermSymbolIndexImpl::SelfValue => todo!(),
        EtherealTermSymbolIndexImpl::SelfLifetime => Some(HirTypeSymbol::SelfLifetime.into()),
        EtherealTermSymbolIndexImpl::SelfPlace => Some(HirTypeSymbol::SelfPlace.into()),
    }
}
