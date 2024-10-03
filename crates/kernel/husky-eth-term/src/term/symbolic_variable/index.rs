use super::*;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_term_prelude::template_var_class::TemplateVariableClass;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EthTemplateVariableAttrs {
    pub class: TemplateVariableClass,
}

impl EthTemplateVariableAttrs {
    pub fn from_dec(attrs: DeclarativeTemplateVariableAttrs) -> Self {
        EthTemplateVariableAttrs { class: attrs.class }
    }

    pub fn phantom(self) -> bool {
        self.class == TemplateVariableClass::Phan
    }
}

impl Into<DeclarativeTemplateVariableAttrs> for EthTemplateVariableAttrs {
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
pub struct EthTermSymbolicVariableIndex(EthTermVariableIndexImpl);

impl EthTermSymbolicVariableIndex {
    pub(super) fn from_dec(index: DecSymbolicVariableIndex) -> Self {
        EthTermSymbolicVariableIndex(match index.inner() {
            DecTermSymbolIndexImpl::ExplicitLifetime {
                attrs,
                variance,
                disambiguator,
            } => EthTermVariableIndexImpl::ExplicitLifetime {
                attrs: EthTemplateVariableAttrs::from_dec(attrs),
                variance,
                disambiguator,
            },
            DecTermSymbolIndexImpl::ExplicitPlace {
                attrs,
                variance,
                disambiguator,
            } => EthTermVariableIndexImpl::ExplicitPlace {
                attrs: EthTemplateVariableAttrs::from_dec(attrs),
                variance,
                disambiguator,
            },
            DecTermSymbolIndexImpl::Type {
                attrs,
                variance,
                disambiguator,
            } => EthTermVariableIndexImpl::Type {
                attrs: EthTemplateVariableAttrs::from_dec(attrs),
                variance,
                disambiguator,
            },
            DecTermSymbolIndexImpl::Prop { disambiguator: _ } => todo!(),
            DecTermSymbolIndexImpl::ConstPathLeading {
                attrs,
                disambiguator,
                ty_path,
            } => EthTermVariableIndexImpl::ConstPathLeading {
                attrs: EthTemplateVariableAttrs::from_dec(attrs),
                disambiguator,
                ty_path,
            },
            DecTermSymbolIndexImpl::ConstOther {
                attrs,
                disambiguator,
            } => EthTermVariableIndexImpl::ConstOther {
                attrs: EthTemplateVariableAttrs::from_dec(attrs),
                disambiguator,
            },
            DecTermSymbolIndexImpl::ConstErr {
                attrs: _,
                disambiguator: _,
            } => todo!(),
            DecTermSymbolIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            } => EthTermVariableIndexImpl::EphemPathLeading {
                disambiguator,
                ty_path,
            },
            DecTermSymbolIndexImpl::EphemOther { disambiguator: _ } => todo!(),
            DecTermSymbolIndexImpl::EphemErr { disambiguator: _ } => todo!(),
            DecTermSymbolIndexImpl::SelfType => EthTermVariableIndexImpl::SelfType,
            DecTermSymbolIndexImpl::SelfValue => EthTermVariableIndexImpl::SelfValue,
            DecTermSymbolIndexImpl::SelfLifetime => EthTermVariableIndexImpl::SelfLifetime,
            DecTermSymbolIndexImpl::SelfPlace => EthTermVariableIndexImpl::SelfPlace,
            DecTermSymbolIndexImpl::AdHoc { disambiguator: _ } => unreachable!(),
        })
    }

    pub fn inner(self) -> EthTermVariableIndexImpl {
        self.0
    }
}

impl Into<DecSymbolicVariableIndex> for EthTermSymbolicVariableIndex {
    #[inline(always)]
    fn into(self) -> DecSymbolicVariableIndex {
        unsafe {
            DecSymbolicVariableIndex::new(match self.inner() {
                EthTermVariableIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator,
                } => DecTermSymbolIndexImpl::ExplicitLifetime {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermVariableIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator,
                } => DecTermSymbolIndexImpl::ExplicitPlace {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermVariableIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator,
                } => DecTermSymbolIndexImpl::Type {
                    attrs: attrs.into(),
                    variance,
                    disambiguator,
                },
                EthTermVariableIndexImpl::Prop { disambiguator } => {
                    DecTermSymbolIndexImpl::Prop { disambiguator }
                }
                EthTermVariableIndexImpl::ConstPathLeading {
                    attrs,
                    disambiguator,
                    ty_path,
                } => DecTermSymbolIndexImpl::ConstPathLeading {
                    attrs: attrs.into(),
                    disambiguator,
                    ty_path,
                },
                EthTermVariableIndexImpl::ConstOther {
                    attrs,
                    disambiguator,
                } => DecTermSymbolIndexImpl::ConstOther {
                    attrs: attrs.into(),
                    disambiguator,
                },
                EthTermVariableIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                } => DecTermSymbolIndexImpl::EphemPathLeading {
                    disambiguator,
                    ty_path,
                },
                EthTermVariableIndexImpl::EphemOther { disambiguator } => {
                    DecTermSymbolIndexImpl::EphemOther { disambiguator }
                }
                EthTermVariableIndexImpl::SelfType => DecTermSymbolIndexImpl::SelfType,
                EthTermVariableIndexImpl::SelfValue => DecTermSymbolIndexImpl::SelfValue,
                EthTermVariableIndexImpl::SelfLifetime => DecTermSymbolIndexImpl::SelfLifetime,
                EthTermVariableIndexImpl::SelfPlace => DecTermSymbolIndexImpl::SelfPlace,
            })
        }
    }
}

// todo: write tests for conversion between declarative and ethereal term symbol index

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[repr(u8)]
pub enum EthTermVariableIndexImpl {
    ExplicitLifetime {
        attrs: EthTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    ExplicitPlace {
        attrs: EthTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Type {
        attrs: EthTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    ConstPathLeading {
        attrs: EthTemplateVariableAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    ConstOther {
        attrs: EthTemplateVariableAttrs,
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
