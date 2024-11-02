mod call_list;
mod separated_list;

pub(super) use self::call_list::*;
pub(super) use self::separated_list::*;

use super::*;
use crate::expr::VdSynExprData;
use expr::{
    list_item::VdSynSeparatedListItem, VdSynBinaryOpr, VdSynLeftDelimiter, VdSynPrefixOpr,
    VdSynSeparator,
};
use latex_token::idx::LxTokenIdx;
use visored_opr::{
    delimiter::VdBaseLeftDelimiter,
    opr::{binary::VdBaseBinaryOpr, prefix::VdBasePrefixOpr},
    precedence::VdPrecedence,
    separator::{VdBaseSeparator, VdSeparator},
};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum IncompleteVdSynExprData {
    Binary {
        lopd: VdSynExprIdx,
        opr: VdSynBinaryOpr,
    },
    Prefix {
        opr: VdSynPrefixOpr,
    },
    /// list separated by commas
    /// ```husky
    /// A(a, b, c)
    /// ```
    SeparatedList {
        separator: VdSeparator,
        fragments: SmallVec<[Either<VdSynExprIdx, VdSynSeparator>; 4]>,
    },
    Delimited {
        bra: VdSynLeftDelimiter,
    },
}

impl IncompleteVdSynExprData {
    pub(super) fn precedence(&self) -> VdPrecedence {
        match self {
            IncompleteVdSynExprData::Binary { opr, .. } => opr.precedence(),
            IncompleteVdSynExprData::Prefix { opr, .. } => todo!(),
            IncompleteVdSynExprData::SeparatedList { separator, .. } => separator.precedence(),
            IncompleteVdSynExprData::Delimited { bra } => todo!(),
        }
    }
}
