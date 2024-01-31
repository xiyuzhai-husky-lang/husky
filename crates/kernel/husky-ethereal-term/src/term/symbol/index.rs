use super::*;
use husky_term_prelude::template_symbol_class::TermTemplateSymbolClass;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EtherealTemplateSymbolAttrs {
    pub class: TermTemplateSymbolClass,
}

impl EtherealTemplateSymbolAttrs {
    pub fn from_declarative(attrs: DeclarativeTemplateSymbolAttrs) -> Self {
        EtherealTemplateSymbolAttrs { class: attrs.class }
    }

    pub fn phantom(self) -> bool {
        self.class == TermTemplateSymbolClass::Phantom
    }
}

impl Into<DeclarativeTemplateSymbolAttrs> for EtherealTemplateSymbolAttrs {
    fn into(self) -> DeclarativeTemplateSymbolAttrs {
        unsafe { DeclarativeTemplateSymbolAttrs::new(self.class) }
    }
}

pub enum EtherealTemplateSymbolAttr {
    Phantom,
}

/// wrapper so such the construction is private
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EtherealTermSymbolIndex(EtherealTermSymbolIndexImpl);

impl EtherealTermSymbolIndex {
    pub(super) fn from_declarative(index: DeclarativeTermSymbolIndex) -> Self {
        EtherealTermSymbolIndex(match index.inner() {
            DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexImpl::ExplicitLifetime {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexImpl::ExplicitPlace {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexImpl::Type {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::Prop { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexImpl::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => EtherealTermSymbolIndexImpl::ConstPathLeading {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                disambiguator,
                ty_path,
            },
            DeclarativeTermSymbolIndexImpl::ConstOther {
                attrs,
                disambiguator,
            } => EtherealTermSymbolIndexImpl::ConstOther {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                disambiguator,
            },
            DeclarativeTermSymbolIndexImpl::ConstErr {
                attrs,
                disambiguator,
            } => todo!(),
            DeclarativeTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            } => EtherealTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            },
            DeclarativeTermSymbolIndexImpl::EphemOther { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexImpl::EphemErr { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexImpl::SelfType => EtherealTermSymbolIndexImpl::SelfType,
            DeclarativeTermSymbolIndexImpl::SelfValue => EtherealTermSymbolIndexImpl::SelfValue,
            DeclarativeTermSymbolIndexImpl::SelfLifetime => {
                EtherealTermSymbolIndexImpl::SelfLifetime
            }
            DeclarativeTermSymbolIndexImpl::SelfPlace => EtherealTermSymbolIndexImpl::SelfPlace,
            DeclarativeTermSymbolIndexImpl::AdHoc { disambiguator } => unreachable!(),
        })
    }

    pub fn inner(self) -> EtherealTermSymbolIndexImpl {
        self.0
    }
}

impl Into<DeclarativeTermSymbolIndex> for EtherealTermSymbolIndex {
    #[inline(always)]
    fn into(self) -> DeclarativeTermSymbolIndex {
        unsafe {
            DeclarativeTermSymbolIndex::new(match self.inner() {
                EtherealTermSymbolIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EtherealTermSymbolIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::ExplicitPlace {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EtherealTermSymbolIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::Type {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EtherealTermSymbolIndexImpl::Prop { disambiguator } => {
                    DeclarativeTermSymbolIndexImpl::Prop { disambiguator }
                }
                EtherealTermSymbolIndexImpl::ConstPathLeading {
                    attrs,
                    disambiguator,
                    ty_path,
                } => DeclarativeTermSymbolIndexImpl::ConstPathLeading {
                    attrs: attrs.into(),
                    disambiguator,
                    ty_path,
                },
                EtherealTermSymbolIndexImpl::ConstOther {
                    attrs,
                    disambiguator,
                } => DeclarativeTermSymbolIndexImpl::ConstOther {
                    attrs: attrs.into(),
                    disambiguator,
                },
                EtherealTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                } => DeclarativeTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                },
                EtherealTermSymbolIndexImpl::EphemOther { disambiguator } => {
                    DeclarativeTermSymbolIndexImpl::EphemOther { disambiguator }
                }
                EtherealTermSymbolIndexImpl::SelfType => DeclarativeTermSymbolIndexImpl::SelfType,
                EtherealTermSymbolIndexImpl::SelfValue => DeclarativeTermSymbolIndexImpl::SelfValue,
                EtherealTermSymbolIndexImpl::SelfLifetime => {
                    DeclarativeTermSymbolIndexImpl::SelfLifetime
                }
                EtherealTermSymbolIndexImpl::SelfPlace => DeclarativeTermSymbolIndexImpl::SelfPlace,
            })
        }
    }
}

// todo: write tests for conversion between declarative and ethereal term symbol index

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[repr(u8)]
pub enum EtherealTermSymbolIndexImpl {
    ExplicitLifetime {
        attrs: EtherealTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    ExplicitPlace {
        attrs: EtherealTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Type {
        attrs: EtherealTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    ConstPathLeading {
        attrs: EtherealTemplateSymbolAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    ConstOther {
        attrs: EtherealTemplateSymbolAttrs,
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
