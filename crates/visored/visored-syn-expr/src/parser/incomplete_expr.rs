mod call_list;
mod separated_list;

pub(super) use self::call_list::*;
pub(super) use self::separated_list::*;

use super::*;
use crate::expr::VdSynExprData;
use expr::{
    list_item::VdSynSeparatedListItem, VdSynBinaryOpr, VdSynExprArenaRef, VdSynLeftDelimiter,
    VdSynPrefixOpr, VdSynSeparator,
};
use latex_token::idx::LxTokenIdx;
use std::fmt::Formatter;
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
        left_delimiter: VdSynLeftDelimiter,
    },
}

impl IncompleteVdSynExprData {
    pub(super) fn precedence(&self) -> VdPrecedence {
        match self {
            IncompleteVdSynExprData::Binary { opr, .. } => opr.precedence(),
            IncompleteVdSynExprData::Prefix { opr, .. } => opr.precedence(),
            IncompleteVdSynExprData::SeparatedList { separator, .. } => separator.precedence(),
            IncompleteVdSynExprData::Delimited { .. } => VdPrecedence::INCOMPLTE_DELIMITED,
        }
    }
}

impl IncompleteVdSynExprData {
    pub fn show(&self, db: &::salsa::Db, arena: VdSynExprArenaRef) -> String {
        match *self {
            IncompleteVdSynExprData::Binary { lopd, opr } => {
                format!("{} {}", arena[lopd].show(db, arena), opr.show(db, arena))
            }
            IncompleteVdSynExprData::Prefix { opr } => opr.show(db, arena),
            IncompleteVdSynExprData::SeparatedList {
                separator,
                ref fragments,
            } => {
                let mut s = "".to_string();
                for fragment in fragments.iter() {
                    if !s.is_empty() {
                        s += " ";
                    }
                    match fragment {
                        Left(expr) => s += &arena[*expr].show(db, arena),
                        Right(sep) => s += &sep.show(db, arena),
                    }
                }
                s
            }
            IncompleteVdSynExprData::Delimited { left_delimiter } => {
                format!("{}", left_delimiter.show(db, arena))
            }
        }
    }
}
