use super::*;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EtherealTemplateSymbolAttrs {
    phantom: bool,
}

impl EtherealTemplateSymbolAttrs {
    pub fn from_declarative(attrs: DeclarativeTemplateSymbolAttrs) -> Self {
        EtherealTemplateSymbolAttrs {
            phantom: attrs.phantom(),
        }
    }
}

impl Into<DeclarativeTemplateSymbolAttrs> for EtherealTemplateSymbolAttrs {
    fn into(self) -> DeclarativeTemplateSymbolAttrs {
        unsafe { DeclarativeTemplateSymbolAttrs::new(self.phantom) }
    }
}

pub enum EtherealTemplateSymbolAttr {
    Phantom,
}

/// wrapper so such the construction is private
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EtherealTermSymbolIndex(EtherealTermSymbolIndexInner);

impl EtherealTermSymbolIndex {
    pub(super) fn from_declarative(index: DeclarativeTermSymbolIndex) -> Self {
        EtherealTermSymbolIndex(match index.inner() {
            DeclarativeTermSymbolIndexInner::Lifetime {
                attrs,
                variance,
                disambiguator,
            } => EtherealTermSymbolIndexInner::Lifetime {
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
                EtherealTermSymbolIndexInner::Lifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => DeclarativeTermSymbolIndexInner::Lifetime {
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
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum EtherealTermSymbolIndexInner {
    Lifetime {
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
}
