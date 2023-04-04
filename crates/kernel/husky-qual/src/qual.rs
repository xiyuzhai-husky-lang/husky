mod part_ident;

pub use self::part_ident::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaseQual {
    mutability: Mutability,
    ownership: Ownership,
}

impl BaseQual {
    pub fn mutability(&self) -> Mutability {
        self.mutability
    }

    pub fn ownership(&self) -> Ownership {
        self.ownership
    }
}

#[test]
fn base_qual_size_works() {
    assert_eq!(
        std::mem::size_of::<BaseQual>(),
        3 * std::mem::size_of::<u8>()
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mutability {
    Immutable,
    Mutable,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Validity {
    Valid,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ownership {
    Owned,
    // Moved Dropped,
    Unowned,
}
