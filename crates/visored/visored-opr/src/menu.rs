use crate::opr::binary::VdBaseBinaryOpr;
use crate::separator::VdBaseSeparator;
use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VdOprMenu {
    pub add: VdBaseSeparator,
    pub eq: VdBaseSeparator,
    pub r#in: VdBaseSeparator,
}

#[salsa::tracked(return_ref)]
pub fn vd_opr_menu(db: &::salsa::Db) -> VdOprMenu {
    VdOprMenu {
        add: VdBaseSeparator::Add,
        eq: VdBaseSeparator::Eq,
        r#in: VdBaseSeparator::In,
    }
}
