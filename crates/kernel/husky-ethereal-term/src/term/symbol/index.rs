use super::*;
use husky_term_prelude::template_symbol_class::TermTemplateSymbolClass;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EthTemplateSymbolAttrs {
    pub class: TermTemplateSymbolClass,
}

impl EthTemplateSymbolAttrs {
    pub fn from_declarative(attrs: DeclarativeTemplateSymbolAttrs) -> Self {
        EthTemplateSymbolAttrs { class: attrs.class }
    }

    pub fn phantom(self) -> bool {
        self.class == TermTemplateSymbolClass::Phantom
    }
}

impl Into<DeclarativeTemplateSymbolAttrs> for EthTemplateSymbolAttrs {
    fn into(self) -> DeclarativeTemplateSymbolAttrs {
        unsafe { DeclarativeTemplateSymbolAttrs::new(self.class) }
    }
}

pub enum EthTemplateSymbolAttr {
    Phantom,
}

/// wrapper so such the construction is private
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EthTermSymbolIndex(EthTermSymbolIndexImpl);

impl EthTermSymbolIndex {
    pub(super) fn from_declarative(index: DeclarativeTermSymbolIndex) -> Self {
        EthTermSymbolIndex(match index.inner() {
            DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => EthTermSymbolIndexImpl::ExplicitLifetime {
                attrs: EthTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => EthTermSymbolIndexImpl::ExplicitPlace {
                attrs: EthTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => EthTermSymbolIndexImpl::Type {
                attrs: EthTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::Prop { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexImpl::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => EthTermSymbolIndexImpl::ConstPathLeading {
                attrs: EthTemplateSymbolAttrs::from_declarative(attrs),
                disambiguator,
                ty_path,
            },
            DeclarativeTermSymbolIndexImpl::ConstOther {
                attrs,
                disambiguator,
            } => EthTermSymbolIndexImpl::ConstOther {
                attrs: EthTemplateSymbolAttrs::from_declarative(attrs),
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::ConstErr {
                attrs,
                disambiguator,
            } => todo!(),
            DeclarativeTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            } => EthTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            },
            DeclarativeTermSymbolIndexImpl::EphemOther { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexImpl::EphemErr { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexImpl::SelfType => EthTermSymbolIndexImpl::SelfType,
            DeclarativeTermSymbolIndexImpl::SelfValue => EthTermSymbolIndexImpl::SelfValue,
            DeclarativeTermSymbolIndexImpl::SelfLifetime => EthTermSymbolIndexImpl::SelfLifetime,
            DeclarativeTermSymbolIndexImpl::SelfPlace => EthTermSymbolIndexImpl::SelfPlace,
            DeclarativeTermSymbolIndexImpl::AdHoc { disambiguator } => unreachable!(),
        })
    }

    pub fn inner(self) -> EthTermSymbolIndexImpl {
        self.0
    }
}

impl Into<DeclarativeTermSymbolIndex> for EthTermSymbolIndex {
    #[inline(always)]
    fn into(self) -> DeclarativeTermSymbolIndex {
        unsafe {
            DeclarativeTermSymbolIndex::new(match self.inner() {
                EthTermSymbolIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermSymbolIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::ExplicitPlace {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermSymbolIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::Type {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermSymbolIndexImpl::Prop { disambiguator } => {
                    DeclarativeTermSymbolIndexImpl::Prop { disambiguator }
                }
                EthTermSymbolIndexImpl::ConstPathLeading {
                    attrs,
                    disambiguator,
                    ty_path,
                } => DeclarativeTermSymbolIndexImpl::ConstPathLeading {
                    attrs: attrs.into(),
                    disambiguator,
                    ty_path,
                },
                EthTermSymbolIndexImpl::ConstOther {
                    attrs,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::ConstOther {
                    attrs: attrs.into(),
                    disambiguator,
                },
                EthTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                } => DeclarativeTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                },
                EthTermSymbolIndexImpl::EphemOther { disambiguator } => {
                    DeclarativeTermSymbolIndexImpl::EphemOther { disambiguator }
                }
                EthTermSymbolIndexImpl::SelfType => DeclarativeTermSymbolIndexImpl::SelfType,
                EthTermSymbolIndexImpl::SelfValue => DeclarativeTermSymbolIndexImpl::SelfValue,
                EthTermSymbolIndexImpl::SelfLifetime => {
                    DeclarativeTermSymbolIndexImpl::SelfLifetime
                }
                EthTermSymbolIndexImpl::SelfPlace => DeclarativeTermSymbolIndexImpl::SelfPlace,
            })
        }
    }
}

// todo: write tests for conversion between declarative and ethereal term symbol index

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[repr(u8)]
pub enum EthTermSymbolIndexImpl {
    ExplicitLifetime {
        attrs: EthTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    ExplicitPlace {
        attrs: EthTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Type {
        attrs: EthTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    ConstPathLeading {
        attrs: EthTemplateSymbolAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    ConstOther {
        attrs: EthTemplateSymbolAttrs,
        disambiguator: u8,
    },
    EphemPathLeading {
        // attrs: TemplateParameterAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    EphemOther {
        // attrs: TemplateParameterAttrs,
        disambiguator: u8,
    },
    SelfType,
    SelfValue,
    SelfLifetime,
    SelfPlace,
}
