use super::*;
use husky_term_prelude::template_var_class::TemplateVariableClass;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EthTemplateSymbolAttrs {
    pub class: TemplateVariableClass,
}

impl EthTemplateSymbolAttrs {
    pub fn from_dec(attrs: DeclarativeTemplateVariableAttrs) -> Self {
        EthTemplateSymbolAttrs { class: attrs.class }
    }

    pub fn phantom(self) -> bool {
        self.class == TemplateVariableClass::Phantom
    }
}

impl Into<DeclarativeTemplateVariableAttrs> for EthTemplateSymbolAttrs {
    fn into(self) -> DeclarativeTemplateVariableAttrs {
        unsafe { DeclarativeTemplateVariableAttrs::new(self.class) }
    }
}

pub enum EthTemplateSymbolAttr {
    Phantom,
}

/// wrapper so such the construction is private
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct EthTermSymbolIndex(EthTermSymbolIndexImpl);

impl EthTermSymbolIndex {
    pub(super) fn from_dec(index: DecSymbolicVariableIndex) -> Self {
        EthTermSymbolIndex(match index.inner() {
            DecTermSymbolIndexImpl::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => EthTermSymbolIndexImpl::ExplicitLifetime {
                attrs: EthTemplateSymbolAttrs::from_dec(attrs),
                variance,
                disambiguator,
            },
            DecTermSymbolIndexImpl::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => EthTermSymbolIndexImpl::ExplicitPlace {
                attrs: EthTemplateSymbolAttrs::from_dec(attrs),
                variance,
                disambiguator,
            },
            DecTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => EthTermSymbolIndexImpl::Type {
                attrs: EthTemplateSymbolAttrs::from_dec(attrs),
                variance,
                disambiguator,
            },
            DecTermSymbolIndexImpl::Prop { disambiguator: _ } => todo!(),
            DecTermSymbolIndexImpl::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => EthTermSymbolIndexImpl::ConstPathLeading {
                attrs: EthTemplateSymbolAttrs::from_dec(attrs),
                disambiguator,
                ty_path,
            },
            DecTermSymbolIndexImpl::ConstOther {
                attrs,
                disambiguator,
            } => EthTermSymbolIndexImpl::ConstOther {
                attrs: EthTemplateSymbolAttrs::from_dec(attrs),
                disambiguator,
            },
            DecTermSymbolIndexImpl::ConstErr {
                attrs: _,
                disambiguator: _,
            } => todo!(),
            DecTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            } => EthTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            },
            DecTermSymbolIndexImpl::EphemOther { disambiguator: _ } => todo!(),
            DecTermSymbolIndexImpl::EphemErr { disambiguator: _ } => todo!(),
            DecTermSymbolIndexImpl::SelfType => EthTermSymbolIndexImpl::SelfType,
            DecTermSymbolIndexImpl::SelfValue => EthTermSymbolIndexImpl::SelfValue,
            DecTermSymbolIndexImpl::SelfLifetime => EthTermSymbolIndexImpl::SelfLifetime,
            DecTermSymbolIndexImpl::SelfPlace => EthTermSymbolIndexImpl::SelfPlace,
            DecTermSymbolIndexImpl::AdHoc { disambiguator: _ } => unreachable!(),
        })
    }

    pub fn inner(self) -> EthTermSymbolIndexImpl {
        self.0
    }
}

impl Into<DecSymbolicVariableIndex> for EthTermSymbolIndex {
    #[inline(always)]
    fn into(self) -> DecSymbolicVariableIndex {
        unsafe {
            DecSymbolicVariableIndex::new(match self.inner() {
                EthTermSymbolIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => DecTermSymbolIndexImpl::ExplicitLifetime {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermSymbolIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator,
                } => DecTermSymbolIndexImpl::ExplicitPlace {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermSymbolIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator,
                } => DecTermSymbolIndexImpl::Type {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermSymbolIndexImpl::Prop { disambiguator } => {
                    DecTermSymbolIndexImpl::Prop { disambiguator }
                }
                EthTermSymbolIndexImpl::ConstPathLeading {
                    attrs,
                    disambiguator,
                    ty_path,
                } => DecTermSymbolIndexImpl::ConstPathLeading {
                    attrs: attrs.into(),
                    disambiguator,
                    ty_path,
                },
                EthTermSymbolIndexImpl::ConstOther {
                    attrs,
                    disambiguator,
                } => DecTermSymbolIndexImpl::ConstOther {
                    attrs: attrs.into(),
                    disambiguator,
                },
                EthTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                } => DecTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                },
                EthTermSymbolIndexImpl::EphemOther { disambiguator } => {
                    DecTermSymbolIndexImpl::EphemOther { disambiguator }
                }
                EthTermSymbolIndexImpl::SelfType => DecTermSymbolIndexImpl::SelfType,
                EthTermSymbolIndexImpl::SelfValue => DecTermSymbolIndexImpl::SelfValue,
                EthTermSymbolIndexImpl::SelfLifetime => DecTermSymbolIndexImpl::SelfLifetime,
                EthTermSymbolIndexImpl::SelfPlace => DecTermSymbolIndexImpl::SelfPlace,
            })
        }
    }
}

// todo: write tests for conversion between declarative and ethereal term symbol index

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
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
