mod r#const;
mod lifetime;
mod place;
mod ty;

pub use self::lifetime::*;
pub use self::place::*;
pub use self::r#const::*;
pub use self::ty::*;

use crate::*;
use husky_eth_term::term::svar::{EthSvar, EthTemplateSymbolAttrs, EthTermSymbolIndexImpl};
use husky_term_prelude::template_var_class::TemplateVarClass;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateVar {
    Type(HirTypeSvar),
    Const(HirConstSvar),
    Lifetime(HirLifetimeSvar),
    Place(HirPlaceSvar),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HirTemplateVarAttrs {
    class: HirTemplateVarClass,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTemplateVarClass {
    Comptime,
    Runtime,
}

impl HirTemplateVarClass {
    fn from_term(class: TemplateVarClass) -> Option<Self> {
        match class {
            TemplateVarClass::Phantom => None,
            TemplateVarClass::Runtime => Some(HirTemplateVarClass::Runtime),
            TemplateVarClass::Comptime => Some(HirTemplateVarClass::Comptime),
        }
    }
}

impl HirTemplateVarAttrs {
    pub(crate) fn from_eth(attrs: EthTemplateSymbolAttrs) -> Option<Self> {
        Some(Self {
            class: HirTemplateVarClass::from_term(attrs.class)?,
        })
    }
}

impl HirTemplateVar {
    pub fn from_eth(symbol: EthSvar, db: &::salsa::Db) -> Option<Self> {
        hir_template_symbol_from_eth(db, symbol)
    }
}

#[salsa::tracked(jar = HirTypeJar)]
fn hir_template_symbol_from_eth(db: &::salsa::Db, symbol: EthSvar) -> Option<HirTemplateVar> {
    match symbol.index(db).inner() {
        EthTermSymbolIndexImpl::ExplicitLifetime {
            attrs,
            variance,
            disambiguator,
        } => Some(
            HirLifetimeSvar {
                attrs: HirTemplateVarAttrs::from_eth(attrs)?,
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
            HirPlaceSvar {
                attrs: HirTemplateVarAttrs::from_eth(attrs)?,
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
                attrs: HirTemplateVarAttrs::from_eth(attrs)?,
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
                HirConstSymbolIndex::PathLeading {
                    attrs: HirTemplateVarAttrs::from_eth(attrs)?,
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
                HirConstSymbolIndex::Other {
                    attrs: HirTemplateVarAttrs::from_eth(attrs)?,
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
