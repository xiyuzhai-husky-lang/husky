mod r#const;
mod lifetime;
mod quary;
mod ty;

pub use self::lifetime::*;
pub use self::quary::*;
pub use self::r#const::*;
pub use self::ty::*;

use crate::*;
use husky_eth_term::term::svar::{EthSvar, EthTemplateSymbolAttrs, EthTermSymbolIndexImpl};
use husky_term_prelude::template_var_class::TemplateSvarClass;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateSvar {
    Type(HirTypeSvar),
    Const(HirConstSvar),
    Lifetime(HirLifetimeSvar),
    Quary(HirQuarySvar),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirTemplateSvarAttrs {
    class: HirTemplateSvarClass,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateSvarClass {
    Comptime,
    Runtime,
}

impl HirTemplateSvarClass {
    fn from_term(class: TemplateSvarClass) -> Option<Self> {
        match class {
            TemplateSvarClass::Phantom => None,
            TemplateSvarClass::Runtime => Some(HirTemplateSvarClass::Runtime),
            TemplateSvarClass::Comptime => Some(HirTemplateSvarClass::Comptime),
        }
    }
}

impl HirTemplateSvarAttrs {
    pub(crate) fn from_eth(attrs: EthTemplateSymbolAttrs) -> Option<Self> {
        Some(Self {
            class: HirTemplateSvarClass::from_term(attrs.class)?,
        })
    }
}

impl HirTemplateSvar {
    pub fn from_eth(symbol: EthSvar, db: &::salsa::Db) -> Option<Self> {
        hir_template_symbol_from_eth(db, symbol)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_template_symbol_from_eth(db: &::salsa::Db, symbol: EthSvar) -> Option<HirTemplateSvar> {
    match symbol.index(db).inner() {
        EthTermSymbolIndexImpl::ExplicitLifetime {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirLifetimeSvar {
                attrs: HirTemplateSvarAttrs::from_eth(attrs)?,
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
            HirQuarySvar {
                attrs: HirTemplateSvarAttrs::from_eth(attrs)?,
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
            HirTypeSvar::Type {
                attrs: HirTemplateSvarAttrs::from_eth(attrs)?,
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
            HirConstSvar::new(
                db,
                HirType::from_eth(symbol.ty(db), db)?,
                HirConstSvarIndex::PathLeading {
                    attrs: HirTemplateSvarAttrs::from_eth(attrs)?,
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
            HirConstSvar::new(
                db,
                HirType::from_eth(symbol.ty(db), db)?,
                HirConstSvarIndex::Other {
                    attrs: HirTemplateSvarAttrs::from_eth(attrs)?,
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
        EthTermSymbolIndexImpl::SelfType => Some(HirTypeSvar::SelfType.into()),
        EthTermSymbolIndexImpl::SelfValue => todo!(),
        EthTermSymbolIndexImpl::SelfLifetime => Some(HirTypeSvar::SelfLifetime.into()),
        EthTermSymbolIndexImpl::SelfPlace => Some(HirTypeSvar::SelfPlace.into()),
    }
}
