use crate::Variance;
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
    Const {
        phantom: bool,
        disambiguator: u8,
        ty_path: TypePath,
    },
    Type {
        phantom: bool,
        variance: Option<Variance>,
        disambiguator: u8,
    },
    Prop {
        disambiguator: u8,
    },
    /// useful when calculatingb application shifts (operad-like)
    AdHoc {
        disambiguator: u8,
    },
    SelfType,
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
    /// only those with the largest disambiguator remains
    cache: Vec<TermSymbolIndex>,
    self_ty_issued: bool,
    self_value_issued: bool,
}

impl TermSymbolRegistry {
    pub fn issue_self_ty_index(&self) -> TermSymbolIndex {
        todo!()
    }

    pub fn issue_self_value_index(&self) -> TermSymbolIndex {
        todo!()
    }

    pub fn issue_lifetime_index(&self) -> TermSymbolIndex {
        todo!()
    }
}
