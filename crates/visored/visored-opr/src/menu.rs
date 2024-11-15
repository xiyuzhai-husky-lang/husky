use crate::opr::binary::VdBaseBinaryOpr;
use crate::separator::VdBaseSeparator;
use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VdOprMenu {
    // ## binary
    pub sub: VdBaseBinaryOpr,
    // ## separator
    pub add: VdBaseSeparator,
    pub space: VdBaseSeparator,
    pub eq: VdBaseSeparator,
    pub le: VdBaseSeparator,
    pub ge: VdBaseSeparator,
    pub r#in: VdBaseSeparator,
}

#[salsa::tracked(return_ref)]
pub fn vd_opr_menu(db: &::salsa::Db) -> VdOprMenu {
    VdOprMenu {
        sub: VdBaseBinaryOpr::SUB,
        add: VdBaseSeparator::ADD,
        space: VdBaseSeparator::SPACE,
        eq: VdBaseSeparator::EQ,
        le: VdBaseSeparator::LE,
        ge: VdBaseSeparator::GE,
        r#in: VdBaseSeparator::IN,
    }
}
