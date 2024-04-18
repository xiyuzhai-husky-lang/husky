use super::*;
use husky_entity_path::TypePath;
use husky_term_prelude::template_var_class::TemplateVariableClass;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DeclarativeTemplateVariableAttrs {
    pub class: TemplateVariableClass,
}

impl DeclarativeTemplateVariableAttrs {
    /// only use this in husky-eth-term
    pub unsafe fn new(class: TemplateVariableClass) -> Self {
        Self { class }
    }

    pub fn from_attrs(attrs: impl IntoIterator<Item = DeclarativeTemplateSymbolAttr>) -> Self {
        let mut slf: Self = Default::default();
        for attr in attrs {
            match attr {
                DeclarativeTemplateSymbolAttr::Phantom => {
                    slf.class = match slf.class {
                        TemplateVariableClass::Phantom => todo!("err"),
                        TemplateVariableClass::Runtime => todo!("err"),
                        TemplateVariableClass::Comptime => TemplateVariableClass::Phantom,
                    }
                }
                DeclarativeTemplateSymbolAttr::Runtime => {
                    slf.class = match slf.class {
                        TemplateVariableClass::Phantom => todo!("err"),
                        TemplateVariableClass::Runtime => todo!("err"),
                        TemplateVariableClass::Comptime => TemplateVariableClass::Runtime,
                    }
                }
            }
        }
        slf
    }
}

pub enum DeclarativeTemplateSymbolAttr {
    Phantom,
    Runtime,
}

/// wrapper so such the construction is private
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DecSymbolicVariableIndex(DecTermSymbolIndexImpl);

impl DecSymbolicVariableIndex {
    pub const SELF_TYPE: Self = Self(DecTermSymbolIndexImpl::SelfType);
    pub const SELF_VALUE: Self = Self(DecTermSymbolIndexImpl::SelfValue);
    pub const SELF_LIFETIME: Self = Self(DecTermSymbolIndexImpl::SelfLifetime);
    pub const SELF_PLACE: Self = Self(DecTermSymbolIndexImpl::SelfPlace);

    /// only use this in husky-eth-term
    pub unsafe fn new(inner: DecTermSymbolIndexImpl) -> Self {
        Self(inner)
    }

    pub unsafe fn new_ad_hoc(disambiguator: u8) -> Self {
        Self(DecTermSymbolIndexImpl::AdHoc { disambiguator })
    }

    pub fn inner(self) -> DecTermSymbolIndexImpl {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum DecTermSymbolIndexImpl {
    ExplicitLifetime {
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    ExplicitPlace {
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Type {
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    ConstPathLeading {
        attrs: DeclarativeTemplateVariableAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    ConstOther {
        attrs: DeclarativeTemplateVariableAttrs,
        disambiguator: u8,
    },
    ConstErr {
        attrs: DeclarativeTemplateVariableAttrs,
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
    EphemErr {
        // attrs: TemplateParameterAttrs,
        disambiguator: u8,
    },
    SelfType,
    SelfValue,
    SelfLifetime,
    SelfPlace,
    /// useful when calculatingb application shifts (operad-like)
    AdHoc {
        disambiguator: u8,
    },
}

#[test]
fn symbol_index_size_works() {
    assert_eq!(
        std::mem::size_of::<DecSymbolicVariableIndex>(),
        std::mem::size_of::<u64>()
    )
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct DecSymbolicVariableRegistry {
    /// cache for issued indices
    ///
    /// only those with the latest disambiguator remains.
    ///
    /// note that they are not next disambiguators that haven't been issued yet.
    cache: Vec<DecSymbolicVariableIndex>,
    self_ty_issued: bool,
    self_value_issued: bool,
}

impl DecSymbolicVariableRegistry {
    pub fn issue_self_ty_index(&mut self) -> DecSymbolicVariableIndex {
        assert!(!self.self_ty_issued);
        self.self_ty_issued = true;
        DecSymbolicVariableIndex(DecTermSymbolIndexImpl::SelfType)
    }

    pub fn issue_self_value_index(&mut self) -> DecSymbolicVariableIndex {
        assert!(!self.self_value_issued);
        self.self_value_issued = true;
        DecSymbolicVariableIndex(DecTermSymbolIndexImpl::SelfValue)
    }

    pub fn issue_ty_index(
        &mut self,
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
    ) -> DecSymbolicVariableIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                DecTermSymbolIndexImpl::Type {
                    attrs: attrs1,
                    variance: variance1,
                    ref mut disambiguator,
                } if attrs1 == attrs && variance1 == variance => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => {
                *latest_disambiguator += 1;
                DecSymbolicVariableIndex(DecTermSymbolIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_explicit_lifetime_index(
        &mut self,
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
    ) -> DecSymbolicVariableIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                DecTermSymbolIndexImpl::ExplicitLifetime {
                    attrs: attrs1,
                    variance: variance1,
                    ref mut disambiguator,
                } if attrs1 == attrs && variance1 == variance => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => {
                *latest_disambiguator += 1;
                DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_explicit_place_index(
        &mut self,
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
    ) -> DecSymbolicVariableIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                DecTermSymbolIndexImpl::ExplicitPlace {
                    attrs: attrs1,
                    variance: variance1,
                    ref mut disambiguator,
                } if attrs1 == attrs && variance1 == variance => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => {
                *latest_disambiguator += 1;
                DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_const_path_leading_index(
        &mut self,
        attrs: DeclarativeTemplateVariableAttrs,
        ty_path: TypePath,
    ) -> DecSymbolicVariableIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DecTermSymbolIndexImpl::ConstPathLeading {
                    attrs: attrs1,
                    disambiguator,
                    ty_path: ty_path1,
                } if attrs1 == attrs && ty_path1 == ty_path => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(_latest_disambiguator) => todo!(),
            None => {
                let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ConstPathLeading {
                    attrs,
                    disambiguator: 0,
                    ty_path,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_const_other_index(
        &mut self,
        attrs: DeclarativeTemplateVariableAttrs,
    ) -> DecSymbolicVariableIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DecTermSymbolIndexImpl::ConstOther {
                    attrs: attrs1,
                    disambiguator,
                } if attrs1 == attrs => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(_latest_disambiguator) => todo!(),
            None => {
                let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ConstOther {
                    attrs,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_const_err_index(
        &mut self,
        attrs: DeclarativeTemplateVariableAttrs,
    ) -> DecSymbolicVariableIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DecTermSymbolIndexImpl::ConstErr {
                    attrs: attrs1,
                    disambiguator,
                } if attrs1 == attrs => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(_latest_disambiguator) => todo!(),
            None => {
                let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::ConstErr {
                    attrs,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_ephem_path_leading_index(
        &mut self,
        // attrs: TemplateParameterAttrs,
        ty_path: TypePath,
    ) -> DecSymbolicVariableIndex {
        let disambiguator = match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DecTermSymbolIndexImpl::EphemPathLeading {
                    // attrs: attrs1,
                    disambiguator,
                    ty_path: ty_path1,
                } if ty_path1 == ty_path => Some(disambiguator),
                // attrs1 == attrs &&
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => latest_disambiguator + 1,
            None => 0,
        };
        let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::EphemPathLeading {
            // attrs,
            disambiguator,
            ty_path,
        });
        self.cache.push(index);
        index
    }

    pub fn issue_ephem_other_index(
        &mut self,
        // attrs: TemplateParameterAttrs
    ) -> DecSymbolicVariableIndex {
        let disambiguator = match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DecTermSymbolIndexImpl::EphemOther {
                    // attrs: attrs1,
                    disambiguator,
                }
                // if attrs1 == attrs 
                => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => latest_disambiguator + 1,
            None => 0,
        };
        let index = DecSymbolicVariableIndex(DecTermSymbolIndexImpl::EphemOther {
            // attrs,
            disambiguator,
        });
        self.cache.push(index);
        index
    }
}
