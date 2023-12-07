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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermSymbolIndex(EtherealTermSymbolIndexInner);

impl EtherealTermSymbolIndex {
    pub(super) fn from_declarative(index: DeclarativeTermSymbolIndex) -> Self {
        EtherealTermSymbolIndex(match index.inner() {
            DeclarativeTermSymbolIndexInner::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexInner::ExplicitLifetime {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexInner::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexInner::ExplicitPlace {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexInner::Type {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexInner::Type {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                variance,
                disambiguator,
            },
            DeclarativeTermSymbolIndexInner::Prop { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexInner::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => EtherealTermSymbolIndexInner::ConstPathLeading {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                disambiguator,
                ty_path,
            },
            DeclarativeTermSymbolIndexInner::ConstOther {
                attrs,
                disambiguator,
            } => EtherealTermSymbolIndexInner::ConstOther {
                attrs: EtherealTemplateSymbolAttrs::from_declarative(attrs),
                disambiguator,
            },
            DeclarativeTermSymbolIndexInner::ConstErr {
                attrs,
                disambiguator,
            } => todo!(),
            DeclarativeTermSymbolIndexInner::EphemPathLeading {
                disambiguator,
                ty_path,
            } => EtherealTermSymbolIndexInner::EphemPathLeading {
                disambiguator,
                ty_path,
            },
            DeclarativeTermSymbolIndexInner::EphemOther { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexInner::EphemErr { disambiguator } => todo!(),
            DeclarativeTermSymbolIndexInner::SelfType => EtherealTermSymbolIndexInner::SelfType,
            DeclarativeTermSymbolIndexInner::SelfValue => EtherealTermSymbolIndexInner::SelfValue,
            DeclarativeTermSymbolIndexInner::SelfLifetime => {
                EtherealTermSymbolIndexInner::SelfLifetime
            }
            DeclarativeTermSymbolIndexInner::SelfPlace => EtherealTermSymbolIndexInner::SelfPlace,
            DeclarativeTermSymbolIndexInner::AdHoc { disambiguator } => unreachable!(),
        })
    }

    pub fn inner(self) -> EtherealTermSymbolIndexInner {
        self.0
    }
}

impl Into<DeclarativeTermSymbolIndex> for EtherealTermSymbolIndex {
    #[inline(always)]
    fn into(self) -> DeclarativeTermSymbolIndex {
        unsafe {
            DeclarativeTermSymbolIndex::new(match self.inner() {
                EtherealTermSymbolIndexInner::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexInner::ExplicitLifetime {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EtherealTermSymbolIndexInner::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexInner::ExplicitPlace {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EtherealTermSymbolIndexInner::Type {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexInner::Type {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EtherealTermSymbolIndexInner::Prop { disambiguator } => todo!(),
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
                EtherealTermSymbolIndexInner::EphemOther { disambiguator } => todo!(),
                EtherealTermSymbolIndexInner::SelfType => todo!(),
                EtherealTermSymbolIndexInner::SelfValue => todo!(),
                EtherealTermSymbolIndexInner::SelfLifetime => todo!(),
                EtherealTermSymbolIndexInner::SelfPlace => todo!(),
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
#[repr(u8)]
pub enum EtherealTermSymbolIndexInner {
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
