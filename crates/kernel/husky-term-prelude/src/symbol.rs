use crate::{template_parameter::TemplateParameterAttrs, Variance};
use husky_entity_path::TypePath;

/// wrapper so such the construction is private
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TermSymbolIndex(TermSymbolIndexInner);

impl TermSymbolIndex {
    pub unsafe fn new_ad_hoc(disambiguator: u8) -> Self {
        Self(TermSymbolIndexInner::AdHoc { disambiguator })
    }

    pub fn inner(self) -> TermSymbolIndexInner {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum TermSymbolIndexInner {
    Lifetime {
        attrs: TemplateParameterAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Type {
        attrs: TemplateParameterAttrs,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    ConstPathLeading {
        attrs: TemplateParameterAttrs,
        disambiguator: u8,
        ty_path: TypePath,
    },
    ConstOther {
        attrs: TemplateParameterAttrs,
        disambiguator: u8,
    },
    ConstErr {
        attrs: TemplateParameterAttrs,
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
    /// useful when calculatingb application shifts (operad-like)
    AdHoc {
        disambiguator: u8,
    },
}

#[test]
fn symbol_index_size_works() {
    assert_eq!(
        std::mem::size_of::<TermSymbolIndex>(),
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
    cache: Vec<TermSymbolIndex>,
    self_ty_issued: bool,
    self_value_issued: bool,
}

impl TermSymbolRegistry {
    pub fn issue_self_ty_index(&mut self) -> TermSymbolIndex {
        assert!(!self.self_ty_issued);
        self.self_ty_issued = true;
        TermSymbolIndex(TermSymbolIndexInner::SelfType)
    }

    pub fn issue_self_value_index(&mut self) -> TermSymbolIndex {
        assert!(!self.self_value_issued);
        self.self_value_issued = true;
        TermSymbolIndex(TermSymbolIndexInner::SelfValue)
    }

    pub fn issue_ty_index(
        &mut self,
        attrs: TemplateParameterAttrs,
        variance: Option<Variance>,
    ) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                TermSymbolIndexInner::Type {
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
                TermSymbolIndex(TermSymbolIndexInner::Type {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::Type {
                    attrs,
                    variance,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_lifetime_index(
        &mut self,
        attrs: TemplateParameterAttrs,
        variance: Option<Variance>,
    ) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.0 {
                TermSymbolIndexInner::Lifetime {
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
                TermSymbolIndex(TermSymbolIndexInner::Lifetime {
                    attrs,
                    variance,
                    disambiguator: *latest_disambiguator,
                })
            }
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::Lifetime {
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
        attrs: TemplateParameterAttrs,
        ty_path: TypePath,
    ) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                TermSymbolIndexInner::ConstPathLeading {
                    attrs: attrs1,
                    disambiguator,
                    ty_path: ty_path1,
                } if attrs1 == attrs && ty_path1 == ty_path => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => todo!(),
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::ConstPathLeading {
                    attrs,
                    disambiguator: 0,
                    ty_path,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_const_other_index(&mut self, attrs: TemplateParameterAttrs) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                TermSymbolIndexInner::ConstOther {
                    attrs: attrs1,
                    disambiguator,
                } if attrs1 == attrs => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => todo!(),
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::ConstOther {
                    attrs,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_const_err_index(&mut self, attrs: TemplateParameterAttrs) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                TermSymbolIndexInner::ConstErr {
                    attrs: attrs1,
                    disambiguator,
                } if attrs1 == attrs => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => todo!(),
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::ConstErr {
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
    ) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                TermSymbolIndexInner::EphemPathLeading {
                    // attrs: attrs1,
                    disambiguator,
                    ty_path: ty_path1,
                } if 
                // attrs1 == attrs &&
                 ty_path1 == ty_path => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => todo!(),
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::EphemPathLeading {
                    // attrs,
                    disambiguator: 0,
                    ty_path,
                });
                self.cache.push(index);
                index
            }
        }
    }

    pub fn issue_ephem_other_index(&mut self, 
        // attrs: TemplateParameterAttrs
    ) -> TermSymbolIndex {
        match self
            .cache
            .iter_mut()
            .filter_map(|index| match index.inner() {
                TermSymbolIndexInner::EphemOther {
                    // attrs: attrs1,
                    disambiguator,
                } 
                // if attrs1 == attrs 
                => Some(disambiguator),
                _ => None,
            })
            .next()
        {
            Some(latest_disambiguator) => todo!(),
            None => {
                let index = TermSymbolIndex(TermSymbolIndexInner::EphemOther {
                    // attrs,
                    disambiguator: 0,
                });
                self.cache.push(index);
                index
            }
        }
    }
}
