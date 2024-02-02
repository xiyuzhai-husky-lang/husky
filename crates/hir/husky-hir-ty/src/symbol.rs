mod r#const;
mod lifetime;
mod place;
mod ty;

pub use self::lifetime::*;
pub use self::place::*;
pub use self::r#const::*;
pub use self::ty::*;

use crate::*;
use husky_ethereal_term::{EthTemplateSymbolAttrs, EthTermSymbolIndexImpl, SymbolEthTerm};
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
    pub(crate) fn from_ethereal(attrs: EthTemplateSymbolAttrs) -> Option<Self> {
        Some(Self {
            class: HirTemplateSymbolClass::from_term(attrs.class)?,
        })
    }
}

impl HirTemplateSymbol {
    pub fn from_ethereal(symbol: SymbolEthTerm, db: &::salsa::Db) -> Option<Self> {
        hir_template_symbol_from_ethereal(db, symbol)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_template_symbol_from_ethereal(
    db: &::salsa::Db,
    symbol: SymbolEthTerm,
) -> Option<HirTemplateSymbol> {
    match symbol.index(db).inner() {
        EthTermSymbolIndexImpl::ExplicitLifetime {
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
        EthTermSymbolIndexImpl::ExplicitPlace {
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
        EthTermSymbolIndexImpl::Type {
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
        EthTermSymbolIndexImpl::Prop { disambiguator: _ } => todo!(),
        EthTermSymbolIndexImpl::ConstPathLeading {
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
        EthTermSymbolIndexImpl::ConstOther {
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
        EthTermSymbolIndexImpl::EphemPathLeading {
            disambiguator: _,
            ty_path: _,
        } => None,
        EthTermSymbolIndexImpl::EphemOther { disambiguator: _ } => None,
        EthTermSymbolIndexImpl::SelfType => Some(HirTypeSymbol::SelfType.into()),
        EthTermSymbolIndexImpl::SelfValue => todo!(),
        EthTermSymbolIndexImpl::SelfLifetime => Some(HirTypeSymbol::SelfLifetime.into()),
        EthTermSymbolIndexImpl::SelfPlace => Some(HirTypeSymbol::SelfPlace.into()),
    }
}
