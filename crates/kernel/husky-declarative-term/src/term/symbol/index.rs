use super::*;
use husky_entity_path::TypePath;
use husky_term_prelude::template_symbol_class::TermTemplateSymbolClass;

// todo: use bitmap?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DeclarativeTemplateSymbolAttrs {
    pub class: TermTemplateSymbolClass,
}

impl DeclarativeTemplateSymbolAttrs {
    /// only use this in husky-ethereal-term
    pub unsafe fn new(class: TermTemplateSymbolClass) -> Self {
        Self { class }
    }

    pub fn from_attrs(attrs: impl IntoIterator<Item = DeclarativeTemplateSymbolAttr>) -> Self {
        let mut slf: Self = Default::default();
        for attr in attrs {
            match attr {
                DeclarativeTemplateSymbolAttr::Phantom => {
                    slf.class = match slf.class {
                        TermTemplateSymbolClass::Phantom => todo!("err"),
                        TermTemplateSymbolClass::Runtime => todo!("err"),
                        TermTemplateSymbolClass::Comptime => TermTemplateSymbolClass::Phantom,
                    }
                }
                DeclarativeTemplateSymbolAttr::Runtime => {
                    slf.class = match slf.class {
                        TermTemplateSymbolClass::Phantom => todo!("err"),
                        TermTemplateSymbolClass::Runtime => todo!("err"),
                        TermTemplateSymbolClass::Comptime => TermTemplateSymbolClass::Runtime,
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
pub struct DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl);

impl DeclarativeTermSymbolIndex {
    pub const SELF_TYPE: Self = Self(DeclarativeTermSymbolIndexImpl::SelfType);
    pub const SELF_VALUE: Self = Self(DeclarativeTermSymbolIndexImpl::SelfValue);
    pub const SELF_LIFETIME: Self = Self(DeclarativeTermSymbolIndexImpl::SelfLifetime);
    pub const SELF_PLACE: Self = Self(DeclarativeTermSymbolIndexImpl::SelfPlace);

    /// only use this in husky-ethereal-term
    pub unsafe fn new(inner: DeclarativeTermSymbolIndexImpl) -> Self {
        Self(inner)
    }

    pub unsafe fn new_ad_hoc(disambiguator: u8) -> Self {
        Self(DeclarativeTermSymbolIndexImpl::AdHoc { disambiguator })
    }

    pub fn inner(self) -> DeclarativeTermSymbolIndexImpl {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum DeclarativeTermSymbolIndexImpl {
    ExplicitLifetime {
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    ExplicitPlace {
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Type {
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    ConstPathLeading {
        attrs: DeclarativeTemplateSymbolAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    ConstOther {
        attrs: DeclarativeTemplateSymbolAttrs,
        disambiguator: u8,
    },
    ConstErr {
        attrs: DeclarativeTemplateSymbolAttrs,
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
        std::mem::size_of::<DeclarativeTermSymbolIndex>(),
        std::mem::size_of::<u64>()
    )
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct TermSymbolRegistry {
    /// cache for issued indices
    ///
    /// only those with the latest disambiguator remains.
    ///
    /// note that they are not next disambiguators that haven't been issued yet.
    cache: Vec<DeclarativeTermSymbolIndex>,
    self_ty_issued: bool,
    self_value_issued: bool,
}

impl TermSymbolRegistry {
    pub fn issue_self_ty_index(&mut self) -> DeclarativeTermSymbolIndex {
        assert!(!self.self_ty_issued);
        self.self_ty_issued = true;
        DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::SelfType)
    }

    pub fn issue_self_value_index(&mut self) -> DeclarativeTermSymbolIndex {
        assert!(!self.self_value_issued);
        self.self_value_issued = true;
        DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::SelfValue)
    }

    pub fn issue_ty_index(
        &mut self,
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
    ) -> DeclarativeTermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                DeclarativeTermSymbolIndexImpl::Type {
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
                DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::Type {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index = DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::Type {
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
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
    ) -> DeclarativeTermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
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
                DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index =
                    DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ExplicitLifetime {
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
        attrs: DeclarativeTemplateSymbolAttrs,
        variance: Option<Variance>,
    ) -> DeclarativeTermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                DeclarativeTermSymbolIndexImpl::ExplicitPlace {
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
                DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ExplicitPlace {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index =
                    DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ExplicitPlace {
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
        attrs: DeclarativeTemplateSymbolAttrs,
        ty_path: TypePath,
    ) -> DeclarativeTermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DeclarativeTermSymbolIndexImpl::ConstPathLeading {
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
                let index =
                    DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ConstPathLeading {
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
        attrs: DeclarativeTemplateSymbolAttrs,
    ) -> DeclarativeTermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DeclarativeTermSymbolIndexImpl::ConstOther {
                    attrs: attrs1,
                    disambiguator,
                } if attrs1 == attrs => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(_latest_disambiguator) => todo!(),
            None => {
                let index =
                    DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ConstOther {
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
        attrs: DeclarativeTemplateSymbolAttrs,
    ) -> DeclarativeTermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DeclarativeTermSymbolIndexImpl::ConstErr {
                    attrs: attrs1,
                    disambiguator,
                } if attrs1 == attrs => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(_latest_disambiguator) => todo!(),
            None => {
                let index = DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::ConstErr {
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
    ) -> DeclarativeTermSymbolIndex {
        let disambiguator = match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DeclarativeTermSymbolIndexImpl::EphemPathLeading {
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
        let index = DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::EphemPathLeading {
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
    ) -> DeclarativeTermSymbolIndex {
        let disambiguator = match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                DeclarativeTermSymbolIndexImpl::EphemOther {
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
        let index = DeclarativeTermSymbolIndex(DeclarativeTermSymbolIndexImpl::EphemOther {
            // attrs,
            disambiguator,
        });
        self.cache.push(index);
        index
    }
}
