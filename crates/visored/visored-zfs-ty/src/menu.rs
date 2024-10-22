use crate::term::literal::{VdZfsLiteral, VdZfsLiteralData};

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfsTypeMenu {
    zero_literal: VdZfsLiteral,
    one_literal: VdZfsLiteral,
    two_literal: VdZfsLiteral,
}

impl VdZfsTypeMenu {
    fn new(db: &::salsa::Db) -> Self {
        Self {
            zero_literal: VdZfsLiteral::new(VdZfsLiteralData::NaturalNumber("0".to_string()), db),
            one_literal: VdZfsLiteral::new(VdZfsLiteralData::NaturalNumber("1".to_string()), db),
            two_literal: VdZfsLiteral::new(VdZfsLiteralData::NaturalNumber("2".to_string()), db),
        }
    }
}

impl VdZfsTypeMenu {
    pub fn zero_literal(&self) -> VdZfsLiteral {
        self.zero_literal
    }

    pub fn one_literal(&self) -> VdZfsLiteral {
        self.one_literal
    }

    pub fn two_literal(&self) -> VdZfsLiteral {
        self.two_literal
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfs_ty_menu(db: &::salsa::Db) -> VdZfsTypeMenu {
    VdZfsTypeMenu::new(db)
}
